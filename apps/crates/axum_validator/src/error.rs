// 错误码

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use log::error;
use serde::Serialize;
use serde_json::json;

/// 错误种类
#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("internal server error")]
    InternalServer,

    #[error("invalid request parameter, {0}")]
    InvalidParameter(String),
    #[error("parse content-type error from header, {0}")]
    HeaderContentType(String),
    #[error("attempt to convert a string to a HeaderValue, {0}")]
    HeaderValue(String),

    // #[from] axum::extract::rejection::PathRejection
    #[error("parse path error, {0}")]
    PathRejection(String),
    // #[from] axum::extract::rejection::QueryRejection
    #[error("parse query error, {0}")]
    QueryRejection(String),
    // #[from] axum::extract::rejection::JsonRejection
    #[error("parse json error, {0}")]
    JsonRejection(String),

    #[error(transparent)]
    ValidationErrors(#[from] validator::ValidationErrors),

    #[error("serde json error, {0}")]
    SerdeJsonError(String),
}

#[derive(Debug, Serialize)]
struct ErrorMsg {
    code: u16,
    msg: String,
}

/// Axum 响应体实现
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let err_msg = ErrorMsg {
            code: StatusCode::BAD_REQUEST.into(),
            msg: self.to_string(),
        };

        let data = serde_json::to_string(&err_msg).unwrap_or_else(|e| {
            error!("转换为JSON字符串失败, error: {:#?}", e);

            json!({
                "code": Into::<u16>::into(StatusCode::INTERNAL_SERVER_ERROR),
                "msg": Error::InternalServer.to_string()
            })
            .to_string()
        });

        let mut resp = Response::new(Body::from(data));
        *resp.status_mut() = StatusCode::OK;

        resp
    }
}
