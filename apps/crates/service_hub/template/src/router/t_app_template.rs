//! 模板管理

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::controller::t_app_template::TemplateController;

/// 路由器
pub struct TemplateRouter;

impl TemplateRouter {
    /// 注册路由
    pub fn register() -> Router {
        Router::new().nest(
            "/app-templates",
            Router::new()
                .route(
                    "/",
                    get(TemplateController::list).post(TemplateController::create),
                )
                .route(
                    "/{id}",
                    get(TemplateController::info)
                        .put(TemplateController::update)
                        .delete(TemplateController::delete),
                )
                .route("/batch_create", post(TemplateController::batch_create))
                .route("/batch_delete", delete(TemplateController::batch_delete))
                .route("/{id}/status", put(TemplateController::update_status)),
        )
    }
}

#[cfg(test)]
mod tests {
    use axum_mock::Error;
    use axum_mock::MockRequest;

    use crate::dto::t_app_template::GetTemplatesResp;
    use entity::template::AppTemplateEntity;

    use super::*;

    #[tokio::test]
    async fn test_router_all() -> Result<(), Error> {
        let mut request = MockRequest::new(TemplateRouter::register())
            .await?
            .from_entity(vec![AppTemplateEntity])
            .await?
            .enabled_log(true);

        let response = request
            .get::<(), GetTemplatesResp>("/app-templates/list", ())
            .await?;
        println!("response: {:#?}", response);

        Ok(())
    }
}
