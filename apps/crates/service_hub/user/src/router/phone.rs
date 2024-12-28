//! 用户手机号管理

use crate::controller::phone::PhoneController;

use axum::{routing::get, Router};

/// 路由器
pub struct PhoneRouter;

impl PhoneRouter {
    /// 注册`用户手机号管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/phones",
            Router::new()
                .route(
                    "/",
                    get(PhoneController::list).post(PhoneController::create),
                )
                .route(
                    "/:id",
                    get(PhoneController::info)
                        .put(PhoneController::update)
                        .delete(PhoneController::delete),
                ),
        )
    }
}
