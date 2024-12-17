//! 接口响应类型
use code::Error;

use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 数据列表
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DataList<T: Serialize> {
    data_list: Vec<T>,
    total: u64,
}

/// 响应结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    /// 返回业务码
    code: u16,
    /// 返回信息
    msg: String,
    /// 返回数据
    data: Option<Value>,
}

impl Response {
    /// 返回成功
    pub fn ok() -> Self {
        Self {
            code: Error::OK.code(),
            msg: Error::OK.msg(),
            data: None,
        }
    }
    /// 设置返回的数据
    pub fn data<T: Serialize>(mut self, data: T) -> Self {
        self.data = Some(json!(data));
        self
    }
    /// 设置返回的数据列表
    pub fn data_list<T: Serialize>(mut self, data_list: Vec<T>, total: u64) -> Self {
        self.data = Some(json!(DataList { data_list, total }));
        self
    }
}

/// 打印 Response
impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(response code: {}, msg: {})", self.code, self.msg)
    }
}

/// Axum 响应体实现
impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
