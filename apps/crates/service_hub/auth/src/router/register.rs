//! 注册用户路由

use crate::controller::register::RegisterController;

use axum::{Router, routing::post};

/// 路由器
pub struct RegisterRouter;

impl RegisterRouter {
    /// 注册`注册用户`路由
    pub fn register() -> Router {
        Router::new().route("/register", post(RegisterController::register))
    }
}
