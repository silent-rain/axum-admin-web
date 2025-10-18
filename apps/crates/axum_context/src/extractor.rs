//! Context extractor.

use crate::Context;

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

impl<S> FromRequestParts<S> for Context
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // ... or use `extract` / `extract_with_state` from `RequestExt` / `RequestPartsExt`
        let context = parts
            .extensions
            .get::<Context>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get context"))?;

        Ok(context.clone()) // 克隆内部的数据
    }
}
