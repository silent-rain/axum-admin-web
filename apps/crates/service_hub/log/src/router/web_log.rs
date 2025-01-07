//! WEB日志管理

use crate::controller::web_log::WebLogController;

use axum::{routing::get, Router};

/// 路由器
pub struct WebLogRouter;

impl WebLogRouter {
    /// 注册`WEB日志管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/web-logs",
            Router::new()
                .route(
                    "/",
                    get(WebLogController::list).post(WebLogController::create),
                )
                .route("/{id}", get(WebLogController::info)),
        )
    }
}
