//! 路由层

pub mod menu;
pub mod menu_role_rel;
pub mod openapi;
pub mod openapi_role_rel;
pub mod token;
pub mod token_role_rel;

use axum::Router;

/// 路由器
pub struct PermissionRouter;

impl PermissionRouter {
    /// 注册`权限管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/permission",
            Router::new()
                .merge(menu::MenuRouter::register()) // 菜单管理
                .merge(menu_role_rel::MenuRoleRelRouter::register()) // 菜单角色关系管理
                .merge(token::TokenRouter::register()) // 令牌管理
                .merge(token_role_rel::TokenRoleRelRouter::register()) // 令牌角色关系管理
                .merge(openapi::OpenapiRouter::register()) // OpenApi接口管理
                .merge(openapi_role_rel::OpenapiRoleRelRouter::register()), // OpenApi接口角色关系管理
        )
    }
}
