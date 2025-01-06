//! 测试
use axum::{
    routing::{get, post},
    Router,
};

use crate::controller::axum_validator::AxumValidatorController;

/// 路由器
pub struct AxumValidatorRouter;

impl AxumValidatorRouter {
    /// 注册路由
    pub fn register() -> Router {
        Router::new().nest(
            "/axum-validators",
            Router::new()
                .route("/say-hello", get(AxumValidatorController::say_hello))
                .route("/say-hi", post(AxumValidatorController::say_hi)),
        )
    }
}
