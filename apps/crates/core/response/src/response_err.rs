//! 异常响应体

use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use code::{Error, ErrorMsg};

/// 异常响应体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseErr {
    /// Return business code
    pub(crate) code: u16,
    /// Return message
    pub(crate) msg: String,
}

impl ResponseErr {
    /// new
    pub fn new(err: Error) -> Self {
        ResponseErr {
            code: err.code(),
            msg: err.msg(),
        }
    }

    ///  Set return msg
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    /// Add msg information and add new information based on the error code information
    pub fn append_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg);
        self
    }
}

/// 将错误枚举转换为响应体
impl From<Error> for ResponseErr {
    fn from(err: Error) -> Self {
        ResponseErr {
            code: err.code(),
            msg: err.msg(),
        }
    }
}

/// 将错误信息转换为响应体
impl From<ErrorMsg> for ResponseErr {
    fn from(err: ErrorMsg) -> Self {
        ResponseErr {
            code: err.code(),
            msg: err.msg().to_string(),
        }
    }
}

/// Axum 响应体实现
impl IntoResponse for ResponseErr {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
