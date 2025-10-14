//! 登陆

use axum::{Router, routing::post};

use crate::controller::login::LoginController;

/// 路由器
pub struct LoginRouter;

impl LoginRouter {
    /// 注册`用户登陆`路由
    pub fn register() -> Router {
        Router::new().route("/login", post(LoginController::login))
    }
}
