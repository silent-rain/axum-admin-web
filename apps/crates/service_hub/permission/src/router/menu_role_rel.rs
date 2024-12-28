//! 菜单角色关系管理

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::controller::menu_role_rel::MenuRoleRelController;

/// 路由器
pub struct MenuRoleRelRouter;

impl MenuRoleRelRouter {
    /// 注册`菜单角色关系管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/menu-role-rels",
            Router::new()
                .route("/", get(MenuRoleRelController::list))
                .route("/batch_create", post(MenuRoleRelController::batch_create))
                .route("/batch_delete", delete(MenuRoleRelController::batch_delete)),
        )
    }
}
