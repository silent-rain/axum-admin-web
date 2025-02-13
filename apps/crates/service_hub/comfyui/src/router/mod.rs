//! 路由层

pub mod task;

use axum::Router;

/// 路由器
pub struct ComfyUIRouter;

impl ComfyUIRouter {
    /// 注册`权限管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/comfyui",
            Router::new().merge(task::ComfyUITaskRouter::register()), // 任务管理
        )
    }
}
