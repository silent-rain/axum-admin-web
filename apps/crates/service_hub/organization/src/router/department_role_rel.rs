//! 部门角色关系管理

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::controller::department_role_rel::DepartmentRoleRelController;

/// 路由器
pub struct DepartmentRoleRelRouter;

impl DepartmentRoleRelRouter {
    /// 注册`部门角色关系管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/department-role-rels",
            Router::new()
                .route("/", get(DepartmentRoleRelController::list))
                .route(
                    "/batch_create",
                    post(DepartmentRoleRelController::batch_create),
                )
                .route(
                    "/batch_delete",
                    delete(DepartmentRoleRelController::batch_delete),
                ),
        )
    }
}
