// 错误码

use axum::{Json, response::IntoResponse};
use serde::Serialize;

/// 错误种类
#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("invalid request parameter, {0}")]
    InvalidParameter(String),
    #[error("parse content-type error from header, {0}")]
    HeaderContentType(String),
    #[error("attempt to convert a string to a HeaderValue, {0}")]
    HeaderValue(String),

    #[error(transparent)]
    ValidateError(#[from] validator::ValidationErrors),

    #[error("serde json error, {0}")]
    SerdeJsonError(String),
}

/// Axum 响应体实现
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
