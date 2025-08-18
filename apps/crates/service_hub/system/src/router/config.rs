//! 配置管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::config::ConfigController;

/// 路由器
pub struct ConfigRouter;

impl ConfigRouter {
    /// 注册`配置管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/configs",
            Router::new()
                .route(
                    "/",
                    get(ConfigController::list).post(ConfigController::create),
                )
                .route("/tree", get(ConfigController::tree))
                .route(
                    "/{id}",
                    get(ConfigController::info)
                        .put(ConfigController::update)
                        .delete(ConfigController::delete),
                )
                .route("/{id}/status", put(ConfigController::update_status)),
        )
    }
}
