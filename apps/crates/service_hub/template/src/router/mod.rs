//! 路由层

use axum::Router;
pub mod simple;
pub mod template;

/// 路由器
pub struct TemplateRouter;

impl TemplateRouter {
    /// 注册`模板管理`路由
    pub fn register() -> Router {
        Router::new()
            .nest("/template", template::AppTemplateRouter::register())
            .nest("/simple", simple::SimpleRouter::register())
    }
}
