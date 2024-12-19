use std::sync::Arc;

use database::{mock::Mock, PoolTrait};
use inject::InjectProvider;
use migration::Migrator;
use response::Response;

use axum::{Extension, Router};
use axum_test::TestServer;
use sea_orm_migration::{migrator::MigratorTrait, MigrationTrait, SchemaManager};
use serde::Serialize;
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("init test server failed, {0}")]
    InitTestServer(String),
    #[error("init db failed, {0}")]
    InitDb(String),
    #[error("BoxBody To Bytes Error, {0}")]
    BoxBodyToBytes(String),
    #[error("Deserialize Bytes Error, {0}")]
    DeserializeBytes(String),
}

pub struct MockRequest {
    pool: Arc<dyn PoolTrait>,
    server: TestServer,
    log_level: tracing::Level,
}

impl MockRequest {
    pub async fn new(routes: Router) -> Result<Self, Error> {
        let pool = Mock::connect().await;

        let provider = Arc::new(InjectProvider::new(pool.clone()));

        // Build an application with a route.
        let app = Router::new().layer(Extension(provider)).merge(routes);

        // Run the application for testing.
        let server = TestServer::new(app).map_err(|err| Error::InitTestServer(err.to_string()))?;

        Ok(MockRequest {
            pool,
            server,
            log_level: tracing::Level::WARN,
        })
    }

    /// 是否启用日志
    pub fn enabled_log(self, enabled: bool) -> Self {
        if !enabled {
            return self;
        }
        tracing_subscriber::fmt()
            .with_max_level(self.log_level)
            .with_level(true)
            .with_line_number(true)
            .init();

        self
    }

    /// 日志级别
    pub fn log_level(mut self, level: tracing::Level) -> Self {
        self.log_level = level;
        self
    }

    /// 迁移库表列表
    pub async fn migrations(self, migrations: Vec<&dyn MigrationTrait>) -> Result<Self, Error> {
        for migration in migrations {
            let manager = SchemaManager::new(self.pool.db());
            migration
                .up(&manager)
                .await
                .map_err(|err| Error::InitDb(err.to_string()))?;
        }
        Ok(self)
    }

    /// 迁移所有库表
    pub async fn all_migrations(self) -> Result<Self, Error> {
        Migrator::up(self.pool.db(), None)
            .await
            .map_err(|err| Error::InitDb(err.to_string()))?;
        Ok(self)
    }

    /// Get 请求
    pub async fn get<T>(&mut self, path: &str, params: T) -> Result<Response<T>, Error>
    where
        T: Serialize,
        T: serde::de::DeserializeOwned,
    {
        self.server.clear_query_params();

        // Get the request.
        self.server.add_query_params(params);

        let response = self.server.get(path).await;

        // Assertions.
        response.assert_status_ok();

        let resp = response.json::<Response<T>>();

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use axum::routing::get;

    use super::*;

    pub async fn health() -> Response<String> {
        Response::<_>::data("ok".to_string())
    }

    pub async fn hello() -> Response<()> {
        Response::<()>::ok()
    }

    #[tokio::test]
    async fn test_mock() -> Result<(), Error> {
        let routes = Router::new()
            .route("/template/health", get(health))
            .route("/template/hello", get(hello));
        let response = MockRequest::new(routes)
            .await?
            .enabled_log(true)
            .get("/template/health", ())
            .await?;
        println!("response: {:#?}", response);

        Ok(())
    }
}
