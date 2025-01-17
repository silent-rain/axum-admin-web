//! 系统接口JWT权限中间件
use std::{boxed::Box, task::Poll};

use axum::{body::Body, extract::Request, http::Response};
use axum_context::{ApiAuthType, Context};
use entity::user::user_login_log;
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::error;

use code::Error;
use jwt::decode_token_with_verify;
use service_hub::{inject::AInjectProvider, user::UserLoginLogService};

use crate::{
    constant::{AUTHORIZATION, AUTHORIZATION_BEARER, AUTH_WHITE_LIST},
    error::create_error_response,
};

/// 系统接口JWT权限中间件
#[derive(Clone)]
pub struct SystemApiJwtAuthLayer;

impl<S> Layer<S> for SystemApiJwtAuthLayer {
    type Service = SystemApiJwtAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SystemApiJwtAuthService { inner }
    }
}

#[derive(Clone)]
pub struct SystemApiJwtAuthService<S> {
    inner: S,
}

impl<S> Service<Request> for SystemApiJwtAuthService<S>
where
    S: Service<Request, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + Sync,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            // 不存在系统鉴权标识时, 则直接通过
            if req.headers().get(AUTHORIZATION).is_none() {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 白名单放行
            let path = req.uri().path();
            if AUTH_WHITE_LIST.contains(&path) {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 获取系统鉴权标识Token
            let system_token = match Self::get_system_api_token(&req) {
                Ok(v) => v,
                Err(err) => {
                    return Ok(create_error_response(err));
                }
            };

            // 全局依赖
            let inject_provider = match req.extensions().get::<AInjectProvider>() {
                Some(v) => v.clone(),
                None => {
                    return Ok(create_error_response(Error::InjectAproviderObj.into_msg()));
                }
            };

            // 验证登陆状态
            match Self::verify_user_login(inject_provider, system_token.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    return Ok(create_error_response(err));
                }
            };

            // 解析系统接口Token
            let user_id = match Self::parse_system_token(system_token.clone()) {
                Ok(v) => v,
                Err(err) => {
                    error!("检查系统鉴权异常, err: {:#?}", err);
                    return Ok(create_error_response(err));
                }
            };

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(user_id);
                ctx.set_api_auth_type(ApiAuthType::System);
            }

            // 响应
            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> SystemApiJwtAuthService<S> {
    /// 获取系统接口鉴权Token
    fn get_system_api_token<ReqBody>(req: &Request<ReqBody>) -> Result<String, code::ErrorMsg> {
        let authorization = req
            .headers()
            .get(AUTHORIZATION)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if authorization.is_empty() {
            error!("鉴权标识为空");
            return Err(code::Error::HeadersNotAuthorization
                .into_msg()
                .with_msg("鉴权标识为空"));
        }
        if !authorization.starts_with(AUTHORIZATION_BEARER) {
            error!(
                "用户请求参数缺失 {AUTHORIZATION_BEARER}, 非法请求, authorization: {authorization}"
            );
            return Err(code::Error::HeadersNotAuthorizationBearer
                .into_msg()
                .with_msg("非法请求"));
        }

        let token = authorization.replace(AUTHORIZATION_BEARER, "");

        Ok(token)
    }

    /// 解析系统接口Token
    fn parse_system_token(token: String) -> Result<i32, code::ErrorMsg> {
        // 解码 Token
        let claims = decode_token_with_verify(&token)
            .map_err(|err| code::Error::TokenDecode(err.to_string()).into_msg())?;
        Ok(claims.user_id)
    }

    /// 验证登陆状态
    async fn verify_user_login(
        provider: AInjectProvider,
        token: String,
    ) -> Result<i32, code::ErrorMsg> {
        let user_login_service: UserLoginLogService = provider.provide();
        let user = user_login_service.info_by_session_id(token.clone()).await?;
        if user.login_status == user_login_log::enums::LoginStatus::Disabled as i8 {
            error!("user_id: {} token: {}, 当前登陆态已被禁用", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("当前登陆态已被禁用, 请重新登陆"));
        }
        if user.login_status == user_login_log::enums::LoginStatus::Failed as i8 {
            error!("user_id: {} token: {}, 无效鉴权", user.id, token.clone());
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("无效鉴权, 请重新登陆"));
        }
        if user.login_status == user_login_log::enums::LoginStatus::Logout as i8 {
            error!("user_id: {} token: {}, 已登出", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("已登出, 请重新登陆"));
        }
        Ok(user.id)
    }
}
