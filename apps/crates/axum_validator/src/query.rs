//! Json extractor.
use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use serde::de::DeserializeOwned;
use tracing::error;
use validator::Validate;

use axum_response::ResponseErr;
use code::Error;

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

#[async_trait]
impl<S, T> FromRequestParts<S> for Query<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = ResponseErr;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 解析查询字符串
        let query_string = &parts.uri.to_string();
        // 从查询字符串中解析出 T 结构体
        let query_info: Result<T, _> = serde_urlencoded::from_str(query_string);
        // 根据解析结果进行验证
        let inner_query = match query_info {
            Ok(v) => v,
            Err(e) => {
                error!("请求参数解析失败, err: {e}");
                return Err(ResponseErr::new(Error::InvalidParameter(e.to_string())));
            }
        };

        // 验证字段
        if let Err(e) = inner_query.validate() {
            error!("请求参数验证失败, err: {e}");
            return Err(ResponseErr::new(Error::ValidateError(e.to_string())));
        }
        Ok(Query(inner_query))
    }
}
