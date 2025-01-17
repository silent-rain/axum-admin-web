//! OpenApi权限中间件
use std::{boxed::Box, task::Poll};

use axum::{body::Body, extract::Request, http::Response};
use axum_context::{ApiAuthType, Context};
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tracing::error;

use code::Error;
use service_hub::{inject::AInjectProvider, permission::TokenService};

use crate::{
    constant::{AUTH_WHITE_LIST, OPENAPI_AUTHORIZATION, OPENAPI_PASSPHRASE},
    error::create_error_response,
};

/// OpenApi接口鉴权
#[derive(Clone)]
pub struct OpenApiAuthLayer;

impl<S> Layer<S> for OpenApiAuthLayer {
    type Service = OpenApiAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        OpenApiAuthService { inner }
    }
}

#[derive(Clone)]
pub struct OpenApiAuthService<S> {
    inner: S,
}

impl<S> Service<Request> for OpenApiAuthService<S>
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
            // 不存在Openapi鉴权标识时, 则直接通过
            if req.headers().get(OPENAPI_AUTHORIZATION).is_none() {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 白名单放行
            let path = req.uri().path();
            if AUTH_WHITE_LIST.contains(&path) {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 获取 Openapi 鉴权标识
            let (openapi_token, passphras) = match Self::get_openapi_token(&req) {
                Ok(v) => v,
                Err(err) => {
                    error!("获取鉴权标识失败, err: {:#?}", err);
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

            // 获取用户权限
            let user_id =
                match Self::get_user_id(inject_provider.clone(), openapi_token.clone(), passphras)
                    .await
                {
                    Ok(v) => v,
                    Err(err) => {
                        error!(
                            "{} 获取用户信息失败, err: {:#?}",
                            openapi_token.clone(),
                            err
                        );
                        return Ok(create_error_response(err));
                    }
                };

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(user_id);
                ctx.set_api_auth_type(ApiAuthType::Openapi);
            }

            // 响应
            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> OpenApiAuthService<S> {
    /// 获取用户ID
    ///
    /// 注意 `info_by_token` 会检查 token 状态
    async fn get_user_id(
        provider: AInjectProvider,
        openapi_token: String,
        passphrase: String,
    ) -> Result<i32, code::ErrorMsg> {
        let token_service: TokenService = provider.provide();
        let user = token_service
            .info_by_token(openapi_token, passphrase)
            .await?;
        Ok(user.user_id)
    }

    /// 获取OPEN API鉴权标识Token
    fn get_openapi_token<ReqBody>(
        req: &Request<ReqBody>,
    ) -> Result<(String, String), code::ErrorMsg> {
        let token = req
            .headers()
            .get(OPENAPI_AUTHORIZATION)
            .map_or("", |v| v.to_str().map_or("", |v| v));

        if token.is_empty() {
            error!("鉴权标识为空");
            return Err(code::Error::HeadersNotAuthorization
                .into_msg()
                .with_msg("鉴权标识为空"));
        }

        let passphras = match req.headers().get(OPENAPI_PASSPHRASE) {
            Some(v) => v.to_str().map_or("", |v| v),
            None => {
                return Err(code::Error::HeadersNotAuthorizationPassphrase
                    .into_msg()
                    .with_msg("鉴权口令不能为空"))
            }
        };

        Ok((token.to_string(), passphras.to_owned()))
    }
}
