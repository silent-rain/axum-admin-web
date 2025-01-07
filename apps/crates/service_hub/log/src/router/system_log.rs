//! 系统日志

use crate::controller::system_log::SystemLogController;

use axum::{routing::get, Router};

/// 路由器
pub struct SystemLogRouter;

impl SystemLogRouter {
    /// 注册`系统日志管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/system-logs",
            Router::new()
                .route(
                    "/",
                    get(SystemLogController::list).post(SystemLogController::create),
                )
                .route(
                    "/{id}",
                    get(SystemLogController::info).delete(SystemLogController::delete),
                ),
        )
    }
}
