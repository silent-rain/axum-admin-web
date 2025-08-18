//! 用户角色关系管理

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::controller::user_role_rel::UserRoleRelController;

/// 路由器
pub struct UserRoleRelRouter;

impl UserRoleRelRouter {
    /// 注册`用户角色关系管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/user-role-rels",
            Router::new()
                .route("/", get(UserRoleRelController::list))
                .route("/batch_create", post(UserRoleRelController::batch_create))
                .route("/batch_delete", delete(UserRoleRelController::batch_delete)),
        )
    }
}
