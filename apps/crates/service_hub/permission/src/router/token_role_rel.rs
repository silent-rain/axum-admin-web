//! 令牌角色关系管理

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::controller::token_role_rel::TokenRoleRelController;

/// 路由器
pub struct TokenRoleRelRouter;

impl TokenRoleRelRouter {
    /// 注册`令牌角色关系管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/token-role-rels",
            Router::new()
                .route("/", get(TokenRoleRelController::list))
                .route("/batch_create", post(TokenRoleRelController::batch_create))
                .route(
                    "/batch_delete",
                    delete(TokenRoleRelController::batch_delete),
                ),
        )
    }
}
