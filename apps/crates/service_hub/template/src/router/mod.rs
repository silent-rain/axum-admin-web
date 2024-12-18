//! 路由层

use axum::Router;
pub mod template;

/// 路由器
pub struct TemplateRouter;

impl TemplateRouter {
    /// 注册`权限管理`路由
    pub fn admin_register() -> Router {
        Router::new().nest("/templates", template::AppTemplateRouter::register())
    }
}
