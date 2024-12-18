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
                "",
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
    use axum_test::TestServer;
    use code::Error;
    use entity::template::AppTemplate;
    // use mock_request::Error;
    // use mock_request::MockRequest;

    use super::*;

    #[tokio::test]
    async fn test_router_all() -> Result<(), Error> {
        // Build an application with a route.
        let app = Router::new().merge(AppTemplateRouter::register());

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Get the request.
        let response = server.get("/ping").await;

        // Assertions.
        response.assert_status_ok();

        let request = MockRequest::new(AppTemplateRouter::register)
            .await
            .migrations(vec![&AppTemplate])
            .await?
            .enabled_log(true);

        let response = request.assert_get("/app-templates/all", ()).await?;
        println!("response: {:#?}", response);

        // 判断业务状态码
        assert!(response.status() == code::Error::OK.code());

        Ok(())
    }
}
