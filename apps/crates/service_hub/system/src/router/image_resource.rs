//! 图片资源管理

use crate::controller::image_resource::ImageResourceController;

use axum::{
    routing::{delete, get, post},
    Router,
};

/// 路由器
pub struct ImageResourceRouter;

impl ImageResourceRouter {
    /// 注册`图片资源管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/image-resources",
            Router::new()
                .route("/", get(ImageResourceController::list))
                .route(
                    "/{id}",
                    get(ImageResourceController::info)
                        .put(ImageResourceController::update)
                        .delete(ImageResourceController::delete),
                )
                .route(
                    "/batch_delete",
                    delete(ImageResourceController::batch_delete),
                )
                .route("/upload", post(ImageResourceController::upload_file))
                .route("/uploads", post(ImageResourceController::upload_files))
                .route("/show_image", get(ImageResourceController::show_image)),
        )
    }
}
