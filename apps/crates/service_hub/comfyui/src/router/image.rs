//! ComfyUI 图片管理

use axum::{
    routing::{get, post},
    Router,
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
                .route(
                    "/upload_mask_image",
                    post(ComfyUIImageController::upload_mask_image),
                )
                .route("/view_image", get(ComfyUIImageController::view_image)),
        )
    }
}
