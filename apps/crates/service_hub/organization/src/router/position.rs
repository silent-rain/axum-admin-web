//! 岗位管理

use crate::controller::position::PositionController;

use axum::{
    routing::{get, put},
    Router,
};

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
