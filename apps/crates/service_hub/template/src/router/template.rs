//! 模板管理

use axum::{
    routing::{delete, get, put},
    Router,
};

use crate::controller::template::AppTemplateController;

/// 路由器
pub struct AppTemplateRouter;

impl AppTemplateRouter {
    /// 注册路由
    pub fn register() -> Router {
        let router = Router::new()
            .route("/all", get(AppTemplateController::all))
            .route(
                "/",
                get(AppTemplateController::list).post(AppTemplateController::add),
            )
            .route(
                "/:id",
                get(AppTemplateController::info)
                    .put(AppTemplateController::update)
                    .delete(AppTemplateController::delete),
            )
            .route("/batch_delete", delete(AppTemplateController::batch_delete))
            .route("/:id/status", put(AppTemplateController::status));

        Router::new().nest("/app-templates", router)
    }
}

#[cfg(test)]
mod tests {
    use axum_mock::Error;
    use axum_mock::MockRequest;
    use entity::template::app_template;
    use entity::template::AppTemplate;
    use response::DataList;

    use super::*;

    #[tokio::test]
    async fn test_router_all() -> Result<(), Error> {
        let mut request = MockRequest::new(AppTemplateRouter::register())
            .await?
            .from_entity(vec![AppTemplate])
            .await?
            .enabled_log(true);

        let response = request
            .get::<(), DataList<app_template::Model>>("/app-templates/all", ())
            .await?;
        println!("response: {:#?}", response);

        Ok(())
    }
}
