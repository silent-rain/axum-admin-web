//! ComfyUI 节点管理

use axum::{routing::get, Router};

use crate::ComfyUINodeController;

/// 路由器
pub struct ComfyUINodeRouter;

impl ComfyUINodeRouter {
    /// 注册`ComfyUI 节点管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/nodes",
            Router::new()
                .route("/object_info", get(ComfyUINodeController::object_info))
                .route("/extensions", get(ComfyUINodeController::extensions)),
        )
    }
}
