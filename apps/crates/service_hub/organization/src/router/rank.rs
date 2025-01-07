//! 职级管理

use crate::controller::rank::RankController;

use axum::{
    routing::{get, put},
    Router,
};

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
