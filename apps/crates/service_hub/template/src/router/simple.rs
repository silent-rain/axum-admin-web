//! 简单示例
use axum::{routing::get, Router};
use axum_extra::extract::cookie::Key;

use crate::controller::simple::SimpleController;

/// 路由器
pub struct SimpleRouter;

impl SimpleRouter {
    /// 注册路由
    pub fn register() -> Router {
        let router = Router::new()
            .route(
                "/check_cookie/{user_id}",
                get(SimpleController::check_cookie).with_state(Key::generate()),
            )
            .route(
                "/user_agent_response",
                get(SimpleController::user_agent_response),
            )
            .route("/user_agent", get(SimpleController::user_agent));

        Router::new().nest("/simples", router)
    }
}
