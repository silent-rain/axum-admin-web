//! 职级管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::rank::RankController;

/// 路由器
pub struct RankRouter;

impl RankRouter {
    /// 注册`职级管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/ranks",
            Router::new()
                .route("/", get(RankController::list).post(RankController::create))
                .route(
                    "/{id}",
                    get(RankController::info)
                        .put(RankController::update)
                        .delete(RankController::delete),
                )
                .route("/{id}/status", put(RankController::update_status)),
        )
    }
}
