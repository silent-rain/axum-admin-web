//! Session权限中间件
use std::{boxed::Box, task::Poll};

use axum::{body::Body, extract::Request, http::Response};
use axum_context::{ApiAuthType, Context};
use futures::future::BoxFuture;
use tower::{Layer, Service};
use tower_sessions::Session;
use tracing::{error, info};

use code::Error;
use entity::user::user_login_log;
use jwt::decode_token_with_verify;
use service_hub::{
    inject::AInjectProvider,
    user::{
        cached::UserCached, dto::user_base::UserPermission, UserBaseService, UserLoginLogService,
    },
};

use crate::{constant::AUTH_WHITE_LIST, error::create_error_response};

/// Session权限中间件
#[derive(Clone)]
pub struct SessionAuthLayer;

impl<S> Layer<S> for SessionAuthLayer {
    type Service = SessionAuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionAuthService { inner }
    }
}

#[derive(Clone)]
pub struct SessionAuthService<S> {
    inner: S,
}

impl<S> Service<Request> for SessionAuthService<S>
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

            // Session
            let session = match req.extensions().get::<Session>() {
                Some(v) => v,
                None => {
                    return Ok(create_error_response(Error::SessionExtension.into_msg()));
                }
            };
            let session_id = match session.id() {
                Some(v) => v.0.to_string(),
                None => {
                    return Ok(create_error_response(
                        Error::SessionIdNotFound
                            .into_msg()
                            .with_msg("权限异常, 请重新登陆"),
                    ))
                }
            };

            // 全局依赖
            let inject_provider = match req.extensions().get::<AInjectProvider>() {
                Some(v) => v.clone(),
                None => {
                    return Ok(create_error_response(Error::InjectAproviderObj.into_msg()));
                }
            };

            // 验证用户登陆状态
            let (user_id, username) =
                match Self::verify_user_login_staus(inject_provider.clone(), session_id.clone())
                    .await
                {
                    Ok(v) => v,
                    Err(err) => {
                        return Ok(create_error_response(err));
                    }
                };

            // 验证用户所在状态

            // 获取缓存
            if let Ok(permission) = UserCached::get_user_system_api_auth(user_id).await {
                // 设置上下文
                if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                    ctx.set_user_id(permission.user_id);
                    ctx.set_user_name(permission.username.clone());
                    ctx.set_role_ids(permission.role_ids.clone());
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

            // 获取用户权限
            let permission = match Self::user_permission(inject_provider.clone(), user_id).await {
                Ok(v) => v,
                Err(err) => {
                    error!("获取权限失败, err: {:#?}", err);
                    return Ok(create_error_response(err));
                }
            };

            // 设置上下文
            if let Some(ctx) = req.extensions_mut().get_mut::<Context>() {
                ctx.set_user_id(permission.user_id);
                // ctx.set_role_ids(user_id);
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

impl<S> SessionAuthService<S> {
    /// 验证用户登陆状态
    ///
    /// TODO 后期可调整为缓存
    async fn verify_user_login_staus(
        provider: AInjectProvider,
        session_id: String,
    ) -> Result<(i32, String), code::ErrorMsg> {
        let user_login_service: UserLoginLogService = provider.provide();
        let user = user_login_service
            .info_by_session_id(session_id.clone())
            .await?;

        if user.login_status == user_login_log::enums::LoginStatus::Disabled as i8 {
            error!(
                "user_id: {} session_id: {}, 当前登陆已被禁用",
                user.id, session_id
            );
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("当前登陆已被禁用, 请重新登陆"));
        }
        if user.login_status == user_login_log::enums::LoginStatus::Failed as i8 {
            error!(
                "user_id: {} session_id: {}, 无效鉴权",
                user.id,
                session_id.clone()
            );
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("登陆失败, 请重新登陆"));
        }
        if user.login_status == user_login_log::enums::LoginStatus::Logout as i8 {
            error!("user_id: {} session_id: {}, 已登出", user.id, session_id);
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("已登出, 请重新登陆"));
        }
        Ok((user.id, user.username))
    }

    /// 检查用户登状态
    ///

    /// 获取用户角色列表
    async fn get_role_ids(
        provider: AInjectProvider,
        user_id: i32,
    ) -> Result<UserPermission, code::ErrorMsg> {
        let user_service: UserBaseService = provider.provide();
        let user = user_service.get_sys_user_permission(user_id).await?;
        Ok(user)
    }
}
