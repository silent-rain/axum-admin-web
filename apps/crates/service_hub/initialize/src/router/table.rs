//! 库表初始化

use crate::controller::table::TableController;

use axum::{routing::get, Router};

/// 路由器
pub struct TableRouter;

impl TableRouter {
    /// 注册`库表初始化`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/tables",
            Router::new().route("/", get(TableController::table)),
        )
    }
}
