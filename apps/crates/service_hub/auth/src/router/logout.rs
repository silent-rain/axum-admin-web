//! 登出

use crate::controller::logout::LogoutController;

use axum::{routing::post, Router};

/// 路由器
pub struct LogoutRouter;

impl LogoutRouter {
    /// 注册`用户登出`路由
    pub fn register() -> Router {
        Router::new().route("/logout", post(LogoutController::logout))
    }
}
