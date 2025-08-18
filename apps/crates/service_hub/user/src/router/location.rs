//! 用户地理位置管理

use axum::{Router, routing::get};

use crate::controller::location::LocationController;

/// 路由器
pub struct LocationRouter;

impl LocationRouter {
    /// 注册`用户地理位置管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/locations",
            Router::new()
                .route(
                    "/",
                    get(LocationController::list).post(LocationController::create),
                )
                .route(
                    "/{id}",
                    get(LocationController::info)
                        .put(LocationController::update)
                        .delete(LocationController::delete),
                ),
        )
    }
}
