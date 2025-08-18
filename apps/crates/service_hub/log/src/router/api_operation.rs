//! API操作日志

use axum::{Router, routing::get};

use crate::controller::api_operation::ApiOperationController;

/// 路由器
pub struct ApiOperationRouter;

impl ApiOperationRouter {
    /// 注册`API操作日志管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/api-operation-logs",
            Router::new()
                .route(
                    "/",
                    get(ApiOperationController::list).post(ApiOperationController::create),
                )
                .route(
                    "/{id}",
                    get(ApiOperationController::info).delete(ApiOperationController::delete),
                ),
        )
    }
}
