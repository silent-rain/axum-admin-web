//! ComfyUI 图片管理

use axum::{
    Router,
    routing::{get, post},
};

use crate::ComfyUIImageController;

/// 路由器
pub struct ComfyUIImageRouter;

impl ComfyUIImageRouter {
    /// 注册`ComfyUI 图片管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/images",
            Router::new()
                .route("/upload_image", post(ComfyUIImageController::upload_image))
                .route("/upload_mask", post(ComfyUIImageController::upload_mask))
                .route(
                    "/upload_image_and_mask",
                    post(ComfyUIImageController::upload_image_and_mask),
                )
                .route("/view_image", get(ComfyUIImageController::view_image)),
        )
    }
}
