//! ComfyUI 系统管理

use axum::{Router, routing::get};

use crate::ComfyUISystemController;

/// 路由器
pub struct ComfyUISystemRouter;

impl ComfyUISystemRouter {
    /// 注册`ComfyUI 系统管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/systems",
            Router::new().route("/system_stats", get(ComfyUISystemController::system_stats)),
        )
    }
}
