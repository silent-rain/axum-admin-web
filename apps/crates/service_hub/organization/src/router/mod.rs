//! 路由层
pub mod department;
pub mod department_role_rel;
pub mod position;
pub mod rank;

use axum::Router;

/// 路由器
pub struct OrganizationRouter;

impl OrganizationRouter {
    /// 注册`组织管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/organization",
            Router::new()
                .merge(department::DepartmentRouter::register()) // 部门管理
                .merge(department_role_rel::DepartmentRoleRelRouter::register()) // 部门角色关系管理
                .merge(position::PositionRouter::register()) // 岗位管理
                .merge(rank::RankRouter::register()), // 职级管理
        )
    }
}
