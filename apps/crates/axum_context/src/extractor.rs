//! Context extractor.

use std::sync::Arc;

use crate::Context;

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use tokio::sync::Mutex;

impl<S> FromRequestParts<S> for Context
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // ... or use `extract` / `extract_with_state` from `RequestExt` / `RequestPartsExt`
        let context = parts
            .extensions
            .get::<Arc<Mutex<Context>>>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get context"))?;

        let context = context.lock().await; // 异步获取锁并解引用
        Ok(context.clone()) // 克隆内部的数据
    }
}
