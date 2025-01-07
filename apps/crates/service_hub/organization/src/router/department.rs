//! 部门管理

use axum::{
    routing::{get, put},
    Router,
};

use crate::controller::department::DepartmentController;

/// 路由器
pub struct DepartmentRouter;

impl DepartmentRouter {
    /// 注册`部门管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/departments",
            Router::new()
                .route(
                    "/",
                    get(DepartmentController::list).post(DepartmentController::create),
                )
                .route("/tree", get(DepartmentController::tree))
                .route(
                    "/{id}",
                    get(DepartmentController::info)
                        .put(DepartmentController::update)
                        .delete(DepartmentController::delete),
                )
                .route("/{id}/status", put(DepartmentController::update_status)),
        )
    }
}
