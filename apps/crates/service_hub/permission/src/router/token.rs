//! 令牌管理

use crate::controller::token::TokenController;

use axum::{
    routing::{get, put},
    Router,
};

/// 路由器
pub struct TokenRouter;

impl TokenRouter {
    /// 注册`令牌管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/tokens",
            Router::new()
                .route(
                    "/",
                    get(TokenController::list).post(TokenController::create),
                )
                .route(
                    "/:id",
                    get(TokenController::info)
                        .put(TokenController::update)
                        .delete(TokenController::delete),
                )
                .route("/:id/status", put(TokenController::update_status)),
        )
    }
}
