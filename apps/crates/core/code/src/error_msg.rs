//! 异常响应体

use serde::{Deserialize, Serialize};

use crate::Error;

/// 异常响应体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorMsg {
    /// Return business code
    pub(crate) code: u16,
    /// Return message
    pub(crate) msg: String,
}

impl ErrorMsg {
    /// new
    pub fn new(err: Error) -> Self {
        ErrorMsg {
            code: err.code(),
            msg: err.msg(),
        }
    }

    /// 返回错误码
    pub fn code(&self) -> u16 {
        self.code
    }

    /// 返回错误信息
    pub fn msg(&self) -> &str {
        &self.msg
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
impl From<Error> for ErrorMsg {
    fn from(err: Error) -> Self {
        ErrorMsg {
            code: err.code(),
            msg: err.msg(),
        }
    }
}

impl Error {
    pub fn into_msg(self) -> ErrorMsg {
        ErrorMsg::from(self)
    }
}
