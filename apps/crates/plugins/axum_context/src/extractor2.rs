//! Context extractor.

use std::sync::Arc;

use crate::Context;

use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use tokio::sync::Mutex;

#[async_trait]
impl<S> FromRequest<S> for Context
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let context = req
            .extensions()
            .get::<Arc<Mutex<Context>>>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get context"))?;

        let context = context.lock().await; // 异步获取锁并解引用
        Ok(context.clone()) // 克隆内部的数据
    }
}
