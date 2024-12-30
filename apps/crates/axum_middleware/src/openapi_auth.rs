//! OpenApi权限中间件
use std::{boxed::Box, convert::Infallible, task::Poll};

use crate::constant::{AUTH_WHITE_LIST, OPENAPI_AUTHORIZATION, OPENAPI_PASSPHRASE};

use code::Error;
use response::ResponseErr;
use service_hub::{
    inject::AInjectProvider,
    user::{cached::UserCached, dto::user_base::UserPermission, UserBaseService},
};

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

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for OpenApiAuthService<S>
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
                Some(v) => &v.0,
                None => {
                    return Err(Into::into(Box::new(ResponseErr::new(Error::InjectAproviderObj))))
                }
            };

            // 白名单放行
            let path = req.uri().path();
            if AUTH_WHITE_LIST.contains(&path) {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 不存在Openapi鉴权标识时, 则直接通过
            if req.headers().get(OPENAPI_AUTHORIZATION).is_none() {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }
            // 获取 Openapi 鉴权
            let (openapi_token, passphras) = match Self::get_openapi_token(&req) {
                Ok(v) => v,
                Err(err) => {
                    error!("获取鉴权标识失败, err: {:#?}", err);
                    return Err(Into::into(Box::new(err)));
                }
            };
            // 获取缓存
            if let Ok(permission) =
                UserCached::get_user_openapi_api_auth(openapi_token.clone()).await
            {
                // 设置上下文
                if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                    ctx.set_user_id(permission.user_id);
                    ctx.set_user_name(permission.username.clone());
                    ctx.set_api_auth_type(ApiAuthType::Openapi);
                }
                info!(
                    "auth user req, cached, auth_type: {:?}, user_id: {}, username: {}",
                    ApiAuthType::Openapi,
                    permission.user_id,
                    permission.username
                );
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 获取用户权限
            let permission = match Self::user_permission(
                inject_provider.clone(),
                openapi_token.clone(),
                passphras,
            )
            .await
            {
                Ok(v) => v,
                Err(err) => {
                    error!("获取权限失败, err: {:#?}", err);
                    return Err(Into::into(Box::new(err)));
                }
            };

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(permission.user_id);
                ctx.set_user_name(permission.username.clone());
                ctx.set_api_auth_type(ApiAuthType::Openapi);
            }
            // 设置缓存
            UserCached::set_user_openapi_api_auth(openapi_token, permission.clone()).await;
            info!(
                "auth user req, auth_type: {:?}, user_id: {}, username: {}",
                ApiAuthType::Openapi,
                permission.user_id,
                permission.username
            );

            // 响应
            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> OpenApiAuthService<S> {
    /// 获取用户权限
    async fn user_permission(
        provider: AInjectProvider,
        openapi_token: String,
        passphrase: String,
    ) -> Result<UserPermission, code::ErrorMsg> {
        let user_service: UserBaseService = provider.provide();
        let user = user_service
            .get_token_user_permission(openapi_token, passphrase)
            .await?;
        Ok(user)
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
