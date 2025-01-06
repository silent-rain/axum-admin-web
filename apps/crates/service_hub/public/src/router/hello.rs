//! 测试
use axum::{routing::get, Router};

use crate::controller::hello::HelloController;

/// 路由器
#[allow(unused)]
pub struct HelloRouter;

impl HelloRouter {
    /// 注册路由
    #[allow(unused)]
    pub fn register() -> Router {
        Router::new().route(
            "/say_hello",
            get(HelloController::say_hello).post(HelloController::say_hi),
        )
    }
}
