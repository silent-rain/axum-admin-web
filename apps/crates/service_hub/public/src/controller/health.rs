//! 健康检查

use response::Response;

use axum::response::IntoResponse;

/// 控制器
pub struct HealthController;

impl HealthController {
    /// 健康检查
    pub async fn health() -> impl IntoResponse {
        Response::data("ok")
    }
}
