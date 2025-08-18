//! 角色管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::role::RoleController;

/// 路由器
pub struct RoleRouter;

impl RoleRouter {
    /// 注册`角色管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/roles",
            Router::new()
                .route("/", get(RoleController::list).post(RoleController::create))
                .route(
                    "/{id}",
                    get(RoleController::info)
                        .put(RoleController::update)
                        .delete(RoleController::delete),
                )
                .route("/{id}/status", put(RoleController::update_status)),
        )
    }
}
