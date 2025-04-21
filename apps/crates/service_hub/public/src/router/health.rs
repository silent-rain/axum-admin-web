//! 健康检查

use axum::{Router, routing::get};

use crate::controller::health::HealthController;

/// 路由器
pub struct HealthRouter;

impl HealthRouter {
    /// 注册路由
    pub fn register() -> Router {
        Router::new().route("/health", get(HealthController::health))
    }
}
