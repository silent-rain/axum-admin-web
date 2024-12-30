//! 路由层

use axum::Router;

pub mod api_operation;
pub mod system_log;
pub mod web_log;

/// 路由器
pub struct LogRouter;

impl LogRouter {
    /// 注册`日志管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/log",
            Router::new()
                .merge(system_log::SystemLogRouter::register()) // 系统日志管理
                .merge(api_operation::ApiOperationRouter::register()) // 操作日志管理
                .merge(web_log::WebLogRouter::register()), // WEB日志管理
        )
    }
}
