//! 健康检查

use axum::{routing::get, Router};

use crate::controller::health::HealthController;

/// 路由器
pub struct HealthRouter;

impl HealthRouter {
    /// 注册路由
    pub fn register() -> Router {
        Router::new().route("/all", get(HealthController::health))
    }
}
