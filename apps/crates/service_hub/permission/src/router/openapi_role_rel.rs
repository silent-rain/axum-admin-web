//! OpenApi接口角色关系管理

use crate::controller::openapi_role_rel::OpenapiRoleRelController;

use axum::{
    routing::{delete, get, post},
    Router,
};

/// 路由器
pub struct OpenapiRoleRelRouter;

impl OpenapiRoleRelRouter {
    /// 注册`OpenApi接口角色关系管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/openapi-role-rels",
            Router::new()
                .route("/", get(OpenapiRoleRelController::list))
                .route(
                    "/batch_create",
                    post(OpenapiRoleRelController::batch_create),
                )
                .route(
                    "/batch_delete",
                    delete(OpenapiRoleRelController::batch_delete),
                ),
        )
    }
}
