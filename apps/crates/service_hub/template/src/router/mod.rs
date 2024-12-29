//! 路由层

use axum::Router;
pub mod simple;
pub mod template;

/// 路由器
pub struct TemplateRouter;

impl TemplateRouter {
    /// 注册`模板管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/template",
            Router::new()
                .merge(template::AppTemplateRouter::register()) // HTTP 模板
                .merge(simple::SimpleRouter::register()), // Grpc 模板
        )
    }
}
