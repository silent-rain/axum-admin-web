//! 路由层

pub mod image;
pub mod model;
pub mod node;
pub mod system;
pub mod task;

use axum::Router;

/// 路由器
pub struct ComfyUIRouter;

impl ComfyUIRouter {
    /// 注册`ComfyUI 管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/comfyui",
            Router::new()
                .merge(task::ComfyUITaskRouter::register()) // ComfyUI 任务管理
                .merge(system::ComfyUISystemRouter::register()) // ComfyUI 系统管理
                .merge(model::ComfyUIModelRouter::register()), // ComfyUI 模型管理
        )
    }
}
