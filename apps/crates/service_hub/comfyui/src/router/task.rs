//! ComfyUI 任务管理

use axum::{
    routing::{get, post},
    Router,
};

use crate::ComfyUITaskController;

/// 路由器
pub struct ComfyUITaskRouter;

impl ComfyUITaskRouter {
    /// 注册`ComfyUI 任务管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/tasks",
            Router::new()
                .route("/push_prompt", post(ComfyUITaskController::push_prompt))
                .route(
                    "/queue_remaining",
                    get(ComfyUITaskController::queue_remaining),
                )
                .route("/historys", get(ComfyUITaskController::historys))
                .route("/history", get(ComfyUITaskController::history))
                .route("/queues", get(ComfyUITaskController::queues))
                .route("/clear_queue", post(ComfyUITaskController::clear_queue))
                .route("/delete_queue", post(ComfyUITaskController::delete_queue))
                .route("/interrupt", post(ComfyUITaskController::interrupt)),
        )
    }
}
