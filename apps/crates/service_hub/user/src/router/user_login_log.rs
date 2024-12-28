//! 登陆日志管理

use crate::controller::user_login_log::UserLoginLogController;

use axum::{routing::get, Router};

/// 路由器
pub struct UserLoginLogRouter;

impl UserLoginLogRouter {
    /// 注册`登陆日志管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/user-login-logs",
            Router::new()
                .route(
                    "/",
                    get(UserLoginLogController::list).post(UserLoginLogController::create),
                )
                .route(
                    "/:id",
                    get(UserLoginLogController::info).put(UserLoginLogController::update),
                )
                .route(
                    "/:id/status",
                    get(UserLoginLogController::info).put(UserLoginLogController::update_status),
                ),
        )
    }
}
