//! Query extractor.
use std::ops::Deref;

use axum::{extract::FromRequestParts, http::request::Parts};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::Error;

#[derive(Debug)]
pub struct Query<T>(pub T);

#[allow(unused)]
impl<T> Query<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Query<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<S, T> FromRequestParts<S> for Query<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 解析查询字符串
        let query_string = &parts.uri.query().unwrap_or_default();
        // 从查询字符串中解析出 T 结构体
        let query_info: Result<T, _> = serde_urlencoded::from_str(query_string);
        // 根据解析结果进行验证
        let inner_query = query_info.map_err(|e| Error::InvalidParameter(e.to_string()))?;

        // 验证字段
        inner_query.validate()?;
        Ok(Query(inner_query))
    }
}
