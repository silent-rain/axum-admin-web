//! OpenApi接口管理

use crate::controller::openapi::OpenapiController;

use axum::{
    routing::{get, put},
    Router,
};

/// 路由器
pub struct OpenapiRouter;

impl OpenapiRouter {
    /// 注册`OpenApi接口管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/openapis",
            Router::new()
                .route(
                    "/",
                    get(OpenapiController::list).post(OpenapiController::create),
                )
                .route(
                    "/:id",
                    get(OpenapiController::info)
                        .put(OpenapiController::update)
                        .delete(OpenapiController::delete),
                )
                .route("/tree", get(OpenapiController::tree))
                .route("/:id/status", put(OpenapiController::update_status)),
        )
    }
}
