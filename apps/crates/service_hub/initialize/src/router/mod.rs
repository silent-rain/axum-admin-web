//! 路由层

use axum::Router;

pub mod table;

/// 路由器
pub struct InitializeRouter;

impl InitializeRouter {
    /// 注册`初始化管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/initialize",
            Router::new().merge(table::TableRouter::register()), // 库表初始化
        )
    }
}
