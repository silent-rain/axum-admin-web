//! 登出

use crate::controller::logout::LogoutController;

use axum::{Router, routing::post};

/// 路由器
pub struct LogoutRouter;

impl LogoutRouter {
    /// 注册`用户登出`路由
    pub fn register() -> Router {
        Router::new().route("/logout", post(LogoutController::logout))
    }
}
