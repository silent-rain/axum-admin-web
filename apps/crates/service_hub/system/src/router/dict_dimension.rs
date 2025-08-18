//! 字典维度管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::dict_dimension::DictDimensionController;

/// 路由器
pub struct DictDimensionRouter;

impl DictDimensionRouter {
    /// 注册`字典维度管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/dict-dimensions",
            Router::new()
                .route(
                    "/",
                    get(DictDimensionController::list).post(DictDimensionController::create),
                )
                .route(
                    "/{id}",
                    get(DictDimensionController::info)
                        .put(DictDimensionController::update)
                        .delete(DictDimensionController::delete),
                )
                .route("/{id}/status", put(DictDimensionController::update_status)),
        )
    }
}
