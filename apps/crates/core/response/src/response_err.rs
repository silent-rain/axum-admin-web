//! 异常响应体

use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use code::Error;

/// 异常响应体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseErr {
    /// 返回业务码
    code: u16,
    /// 返回信息
    msg: String,
}

impl ResponseErr {
    /// 重置错误信息
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    /// 追加错误信息, 在错误码信息的基础上添加新的信息
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

/// Axum 响应体实现
impl IntoResponse for ResponseErr {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
