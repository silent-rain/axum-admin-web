use std::sync::Arc;

use axum_response::ResponseErr;

use crate::Error;

#[derive(Debug, thiserror::Error)]
pub struct ErrorMsg {
    code: u16,
    msg: String,
    inner: Option<Arc<Error>>,
}

impl ErrorMsg {
    /// 创建一个新的错误信息
    pub fn new(code: u16, msg: &str) -> Self {
        Self {
            code,
            msg: msg.to_string(),
            inner: None,
        }
    }

    /// 从 Error 创建一个新的错误信息
    pub fn form_err(err: Error) -> Self {
        Self {
            code: err.code(),
            msg: err.msg(),
            inner: Some(err.into()),
        }
    }

    /// 重置错误码
    pub fn with_code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    /// 重置错误信息
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    /// 追加错误信息, 在错误码信息的基础上添加新的信息
    pub fn with_append_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg);
        self
    }

    /// 返回错误码
    pub fn code(&self) -> u16 {
        self.code
    }

    /// 返回错误信息
    pub fn msg(&self) -> &str {
        &self.msg
    }

    /// 返回内部错误信息
    pub fn inner_err(&self) -> Option<Arc<Error>> {
        self.inner.clone()
    }
}

impl std::fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorMsg(code: {}, msg: {})", self.code(), self.msg())
    }
}

impl From<ErrorMsg> for ResponseErr {
    fn from(err_msg: ErrorMsg) -> ResponseErr {
        ResponseErr::new(err_msg.code(), err_msg.msg())
    }
}
