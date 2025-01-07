//! 文件资源管理

use crate::controller::file_resource::FileResourceController;

use axum::{
    routing::{delete, get, post},
    Router,
};

/// 路由器
pub struct FileResourceRouter;

impl FileResourceRouter {
    /// 注册`文件资源管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/file-resources",
            Router::new()
                .route("/", get(FileResourceController::list))
                .route(
                    "/{id}",
                    get(FileResourceController::info)
                        .put(FileResourceController::update)
                        .delete(FileResourceController::delete),
                )
                .route(
                    "/batch_delete",
                    delete(FileResourceController::batch_delete),
                )
                .route("/upload", post(FileResourceController::upload_file))
                .route("/uploads", post(FileResourceController::upload_files))
                .route("/show_image", get(FileResourceController::show_image)),
        )
    }
}
