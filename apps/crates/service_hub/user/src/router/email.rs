//! 用户邮箱管理

use crate::controller::email::EmailController;

use axum::{routing::get, Router};

/// 路由器
pub struct EmailRouter;

impl EmailRouter {
    /// 注册`用户邮箱管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/emails",
            Router::new()
                .route(
                    "/",
                    get(EmailController::list).post(EmailController::create),
                )
                .route(
                    "/{id}",
                    get(EmailController::info)
                        .put(EmailController::update)
                        .delete(EmailController::delete),
                ),
        )
    }
}
