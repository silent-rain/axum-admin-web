//! 岗位管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::position::PositionController;

/// 路由器
pub struct PositionRouter;

impl PositionRouter {
    /// 注册`岗位管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/positions",
            Router::new()
                .route(
                    "/",
                    get(PositionController::list).post(PositionController::create),
                )
                .route(
                    "/{id}",
                    get(PositionController::info)
                        .put(PositionController::update)
                        .delete(PositionController::delete),
                )
                .route("/{id}/status", put(PositionController::update_status)),
        )
    }
}
