//! 会员等级管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::member_level::MemberLevelController;

/// 路由器
pub struct MemberLevelRouter;

impl MemberLevelRouter {
    /// 注册`会员等级管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/member-levels",
            Router::new()
                .route(
                    "/",
                    get(MemberLevelController::list).post(MemberLevelController::create),
                )
                .route(
                    "/{id}",
                    get(MemberLevelController::info)
                        .put(MemberLevelController::update)
                        .delete(MemberLevelController::delete),
                )
                .route("/{id}/status", put(MemberLevelController::update_status)),
        )
    }
}
