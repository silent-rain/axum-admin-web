//! API操作日志表

use serde::{Deserialize, Serialize};

/// Api 操作日志类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HttpType {
    /// 请求
    #[serde(rename = "REQ")]
    Req,
    /// 响应
    #[serde(rename = "RESP")]
    Resp,
}

impl From<HttpType> for String {
    fn from(value: HttpType) -> Self {
        match value {
            HttpType::Req => "REQ".to_owned(),
            HttpType::Resp => "RESP".to_owned(),
        }
    }
}
