//! 部门角色关系管理

use crate::controller::department_role_rel::DepartmentRoleRelController;

use axum::{
    routing::{delete, get, post},
    Router,
};
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
