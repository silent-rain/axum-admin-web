//! JWT权限中间件
use std::{boxed::Box, task::Poll};

use axum::{body::Body, extract::Request, http::Response};
use axum_context::{ApiAuthType, Context};
use futures::future::BoxFuture;
use log::error;
use tower::{Layer, Service};

use crate::{
    Claims, Error,
    constant::{AUTH_WHITE_LIST, AUTHORIZATION, AUTHORIZATION_BEARER},
    error::ErrorMsg,
};

/// JWT权限中间件
#[derive(Clone)]
pub struct JwtAuthLayer;

impl<S> Layer<S> for JwtAuthLayer {
    type Service = JwtAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        JwtAuthService { inner }
    }
}

#[derive(Clone)]
pub struct JwtAuthService<S> {
    inner: S,
}

impl<S> Service<Request> for JwtAuthService<S>
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
                    return Ok(err.into());
                }
            };

            // 解析系统接口Token
            let claims = match Self::parse_system_token(system_token.clone()) {
                Ok(v) => v,
                Err(err) => {
                    error!("检查系统鉴权异常, err: {err:?}");
                    return Ok(err.into());
                }
            };

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(claims.user_id);
                ctx.set_user_name(claims.username);
                ctx.set_api_auth_type(ApiAuthType::System);
            }

            // 响应
            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> JwtAuthService<S> {
    /// 获取系统接口鉴权Token
    fn get_system_api_token<ReqBody>(req: &Request<ReqBody>) -> Result<String, ErrorMsg> {
        let authorization = req
            .headers()
            .get(AUTHORIZATION)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if authorization.is_empty() {
            error!("鉴权标识为空");
            return Err(Error::HeadersNotAuthorization.into_err());
        }
        if !authorization.starts_with(AUTHORIZATION_BEARER) {
            error!(
                "用户请求参数缺失 {AUTHORIZATION_BEARER}, 非法请求, authorization: {authorization}"
            );
            return Err(Error::HeadersNotAuthorizationBearer.into_err());
        }

        let token = authorization.replace(AUTHORIZATION_BEARER, "");

        Ok(token)
    }

    /// 解析系统接口Token
    fn parse_system_token(token: String) -> Result<Claims, ErrorMsg> {
        // 解码 Token
        let claims = Claims::decode_token(&token)?;
        // 验证 Token
        claims.verify()?;

        Ok(claims)
    }
}
