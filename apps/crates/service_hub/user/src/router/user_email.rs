//! 用户邮箱管理

use axum::{Router, routing::get};

use crate::controller::user_email::EmailController;

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
