//! 系统日志

use crate::controller::system::SystemController;

use axum::{routing::get, Router};

/// 路由器
pub struct SystemRouter;

impl SystemRouter {
    /// 注册`系统日志管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/system-logs",
            Router::new()
                .route(
                    "/",
                    get(SystemController::list).post(SystemController::create),
                )
                .route(
                    "/:id",
                    get(SystemController::info).delete(SystemController::delete),
                ),
        )
    }
}
