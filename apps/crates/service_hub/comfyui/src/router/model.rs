//! ComfyUI 模型管理

use axum::{Router, routing::get};

use crate::ComfyUIModelController;

/// 路由器
pub struct ComfyUIModelRouter;

impl ComfyUIModelRouter {
    /// 注册`ComfyUI 模型管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/models",
            Router::new()
                .route("/embeddings", get(ComfyUIModelController::embeddings))
                .route("/checkpoints", get(ComfyUIModelController::checkpoints))
                .route("/loras", get(ComfyUIModelController::loras))
                .route("/vaes", get(ComfyUIModelController::vaes))
                .route("/clip_visions", get(ComfyUIModelController::clip_visions))
                .route("/controlnets", get(ComfyUIModelController::controlnets))
                .route(
                    "/upscale_models",
                    get(ComfyUIModelController::upscale_models),
                )
                .route("/ipadapters", get(ComfyUIModelController::ipadapters))
                .route("/unet_ggufs", get(ComfyUIModelController::unet_ggufs)),
        )
    }
}
