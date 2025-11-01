//! 错误码
#![allow(unused)]
use axum::{
    body::Body,
    http::{Response, StatusCode},
};
use log::error;
use serde::{Serialize, Serializer};
use serde_json::json;

#[derive(Debug, PartialEq, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    #[error("internal server error")]
    InternalServer,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<Error> for Response<Body> {
    fn from(err: Error) -> Self {
        let err_msg = ErrorMsg {
            code: StatusCode::BAD_REQUEST.into(),
            msg: err.to_string(),
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

#[derive(Debug, Serialize)]
struct ErrorMsg {
    code: u16,
    msg: String,
}
