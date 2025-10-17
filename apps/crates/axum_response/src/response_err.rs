//! 异常响应体

use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

/// 异常响应体
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ResponseErr {
    /// Return business code
    pub(crate) code: u16,
    /// Return message
    pub(crate) msg: String,
}

impl ResponseErr {
    pub fn new(code: u16, msg: &str) -> Self {
        ResponseErr {
            code,
            msg: msg.to_string(),
        }
    }

    /// Set code
    pub fn with_code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    /// Set msg
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    /// Add msg information and add new information based on the error code information
    pub fn with_append_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg);
        self
    }
}

impl std::fmt::Display for ResponseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResponseErr(code: {}, msg: {})", self.code, self.msg)
    }
}

impl std::error::Error for ResponseErr {}

/// Axum 响应体实现
impl IntoResponse for ResponseErr {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl From<serde_json::Error> for ResponseErr {
    fn from(err: serde_json::Error) -> ResponseErr {
        ResponseErr::new(500, &err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_err() {
        let resp = ResponseErr::new(0, "error")
            .with_code(500)
            .with_msg("error")
            .with_append_msg("append msg");
        println!("resp: {:#?}", resp);

        println!("resp to_string: {:#?}", resp.to_string());

        let resp_str =
            serde_json::to_string_pretty(&resp).expect("Failed to serialize response to JSON");
        println!("resp str: {}", resp_str);
    }
}
