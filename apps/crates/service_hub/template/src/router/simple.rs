//! 简单示例
use axum::{
    Router,
    routing::{get, post},
};
use axum_extra::extract::cookie::Key;

use crate::controller::simple::SimpleController;

/// 路由器
pub struct SimpleRouter;

impl SimpleRouter {
    /// 注册路由
    pub fn register() -> Router {
        let router = Router::new()
            .route("/say-hello", get(SimpleController::say_hello))
            .route("/say-hi", post(SimpleController::say_hi))
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
