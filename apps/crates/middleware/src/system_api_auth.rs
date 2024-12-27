//! 系统接口权限中间件
use std::{boxed::Box, convert::Infallible, task::Poll};

use axum::{
    body::{Body, HttpBody},
    http::Request,
    BoxError, Extension,
};
use axum_context::{ApiAuthType, Context};
use bytes::Bytes;
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::{error, info};

use code::Error;
use entity::user::user_login_log;
use jwt::decode_token_with_verify;
use response::ResponseErr;
use service_hub::inject::{
    log::UserLoginService,
    user::{cached::UserCached, dto::user_base::UserPermission, UserBaseService},
    AInjectProvider,
};

use crate::constant::{AUTH_WHITE_LIST, SYSTEM_API_AUTHORIZATION, SYSTEM_API_AUTHORIZATION_BEARER};

/// 系统接口权限中间件
#[derive(Clone)]
pub struct SystemApiAuthLayer;

impl<S> Layer<S> for SystemApiAuthLayer {
    type Service = SystemApiAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SystemApiAuthService { inner }
    }
}

#[derive(Clone)]
pub struct SystemApiAuthService<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for SystemApiAuthService<S>
where
    S: Service<Request<ReqBody>, Response = axum::response::Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    Infallible: From<<S as Service<Request<ReqBody>>>::Error>,
    ResBody: HttpBody<Data = Bytes> + Send + 'static + From<Body>,
    ResBody::Error: Into<BoxError>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let not_ready_inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, not_ready_inner);

        Box::pin(async move {
            // 全局依赖
            let inject_provider = match req.extensions().get::<Extension<AInjectProvider>>() {
                Some(v) => v.0,
                None => {
                    return Err(Box::new(ResponseErr::new(Error::InjectAproviderObj)))
                        .map_err(Into::into)
                }
            };

            // 白名单放行
            let path = req.uri().path();
            if AUTH_WHITE_LIST.contains(&path) {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 不存在系统鉴权标识时, 则直接通过
            if req.headers().get(SYSTEM_API_AUTHORIZATION).is_none() {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 获取系统鉴权标识Token
            let system_token = match Self::get_system_api_token(&req) {
                Ok(v) => v,
                Err(err) => {
                    error!("获取系统鉴权标识 Token 失败, err: {:#?}", err);
                    return Err(Box::new(err.into())).map_err(Into::into);
                }
            };
            // 解析系统接口Token
            let (user_id, _) = match Self::parse_system_token(system_token.clone()) {
                Ok(v) => v,
                Err(err) => {
                    error!("检查系统鉴权异常, err: {:#?}", err);
                    return Err(Box::new(err.into())).map_err(Into::into);
                }
            };
            // 获取缓存
            if let Ok(permission) = UserCached::get_user_system_api_auth(user_id).await {
                // 设置上下文
                if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                    ctx.set_user_id(permission.user_id);
                    ctx.set_user_name(permission.username.clone());
                    ctx.set_api_auth_type(ApiAuthType::System);
                }
                info!(
                    "auth user req, cached, auth_type: {:?}, user_id: {}, username: {}",
                    ApiAuthType::System,
                    permission.user_id,
                    permission.username
                );
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 验证登陆状态
            let user_login_id =
                match Self::verify_user_login(inject_provider.clone(), system_token).await {
                    Ok(v) => v,
                    Err(err) => return Err(Box::new(err.into())).map_err(Into::into),
                };
            // 获取用户权限
            let permission = match Self::user_permission(inject_provider, user_id).await {
                Ok(v) => v,
                Err(err) => {
                    error!("获取权限失败, err: {:#?}", err);
                    return Err(Box::new(err.into())).map_err(Into::into);
                }
            };

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(permission.user_id);
                ctx.set_user_login_id(user_login_id);
                ctx.set_user_name(permission.username.clone());
                ctx.set_api_auth_type(ApiAuthType::System);
            }
            // 设置缓存
            UserCached::set_user_system_api_auth(user_id, permission.clone()).await;
            info!(
                "auth user req, auth_type: {:?}, user_id: {}, username: {}",
                ApiAuthType::System,
                permission.user_id,
                permission.username
            );

            // 响应
            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> SystemApiAuthService<S> {
    /// 解析系统接口Token
    fn parse_system_token(token: String) -> Result<(i32, String), code::Error> {
        // 解码 Token
        let claims = decode_token_with_verify(&token)
            .map_err(|err| code::Error::TokenDecode(err.to_string()))?;
        Ok((claims.user_id, claims.username))
    }

    /// 获取系统接口鉴权Token
    fn get_system_api_token<ReqBody>(req: &Request<ReqBody>) -> Result<String, code::ErrorMsg> {
        let authorization = req
            .headers()
            .get(SYSTEM_API_AUTHORIZATION)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if authorization.is_empty() {
            error!("鉴权标识为空");
            return Err(code::Error::HeadersNotAuthorization
                .into_msg()
                .with_msg("鉴权标识为空"));
        }
        if !authorization.starts_with(SYSTEM_API_AUTHORIZATION_BEARER) {
            error!(
                "用户请求参数缺失 {SYSTEM_API_AUTHORIZATION_BEARER}, 非法请求, authorization: {authorization}"
            );
            return Err(code::Error::HeadersNotAuthorizationBearer
                .into_msg()
                .with_msg("非法请求"));
        }

        let token = authorization.replace(SYSTEM_API_AUTHORIZATION_BEARER, "");

        Ok(token)
    }

    /// 获取用户权限
    async fn user_permission(
        provider: AInjectProvider,
        user_id: i32,
    ) -> Result<UserPermission, code::ErrorMsg> {
        let user_service: UserBaseService = provider.provide();
        let user = user_service.get_sys_user_permission(user_id).await?;
        Ok(user)
    }

    /// 验证登陆状态
    /// TODO 后期可调整为缓存
    async fn verify_user_login(
        provider: AInjectProvider,
        token: String,
    ) -> Result<i32, code::ErrorMsg> {
        let user_login_service: UserLoginService = provider.provide();
        let user = user_login_service.info_by_token(token.clone()).await?;
        if user.status == user_login_log::enums::Status::Disabled as i8 {
            error!("user_id: {} token: {}, 当前登陆态已被禁用", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("当前登陆态已被禁用, 请重新登陆"));
        }
        if user.status == user_login_log::enums::Status::Failed as i8 {
            error!("user_id: {} token: {}, 无效鉴权", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("无效鉴权, 请重新登陆"));
        }
        if user.status == user_login_log::enums::Status::Logout as i8 {
            error!("user_id: {} token: {}, 已登出", user.id, token);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("已登出, 请重新登陆"));
        }
        Ok(user.id)
    }
}
