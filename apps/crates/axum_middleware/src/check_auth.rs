//! 权限检查中间件
use std::{boxed::Box, task::Poll};

use axum::{body::Body, extract::Request, http::Response};
use futures::future::BoxFuture;
use tower::{Layer, Service};

use axum_context::Context;
use code::Error;
use service_hub::{
    inject::AInjectProvider,
    user::{dto::user_base::RolesReq, UserBaseService},
};

use crate::{constant::AUTH_WHITE_LIST, error::create_error_response};

/// 权限检查中间件
#[derive(Clone)]
pub struct CheckAuthLayer;

impl<S> Layer<S> for CheckAuthLayer {
    type Service = CheckAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CheckAuthService { inner }
    }
}

#[derive(Clone)]
pub struct CheckAuthService<S> {
    inner: S,
}

impl<S> Service<Request> for CheckAuthService<S>
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
            // 白名单放行
            let path = req.uri().path();
            if AUTH_WHITE_LIST.contains(&path) {
                let resp = inner.call(req).await?;
                return Ok(resp);
            }

            // 全局依赖
            let inject_provider = match req.extensions().get::<AInjectProvider>() {
                Some(v) => v.clone(),
                None => {
                    return Ok(create_error_response(Error::InjectAproviderObj.into_msg()));
                }
            };

            // 获取上下文
            let ctx = match req.extensions_mut().get_mut::<Context>() {
                Some(ctx) => ctx,
                None => {
                    let resp = inner.call(req).await?;
                    return Ok(resp);
                }
            };

            // 设置上下文
            let user_id = ctx.get_user_id();

            // 用户校验
            let (_, username) = match Self::get_user_checked(inject_provider.clone(), user_id).await
            {
                Ok(v) => v,
                Err(err) => {
                    return Ok(create_error_response(err));
                }
            };
            ctx.set_user_name(username);

            // 获取角色
            let role_ids = match Self::get_role(inject_provider, user_id).await {
                Ok(v) => v,
                Err(err) => {
                    return Ok(create_error_response(err));
                }
            };
            ctx.set_role_ids(role_ids);

            // 响应
            let resp = inner.call(req).await?;
            Ok(resp)
        })
    }
}

impl<S> CheckAuthService<S> {
    /// 用户校验
    async fn get_user_checked(
        provider: AInjectProvider,
        user_id: i32,
    ) -> Result<(i32, String), code::ErrorMsg> {
        let user_base_service: UserBaseService = provider.provide();
        let user = user_base_service.info_checked(user_id).await?;

        Ok((user.id, user.username))
    }

    /// 获取角色
    async fn get_role(provider: AInjectProvider, user_id: i32) -> Result<Vec<i32>, code::ErrorMsg> {
        let user_base_service: UserBaseService = provider.provide();
        let (roles, _) = user_base_service.roles(RolesReq { user_id }).await?;

        let mut role_ids = Vec::new();
        for item in roles {
            role_ids.push(item.id);
        }

        Ok(role_ids)
    }
}
