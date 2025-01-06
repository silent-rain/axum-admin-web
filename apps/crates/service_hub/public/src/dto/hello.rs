//! 测试

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 添加数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct HelloReq {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloResp {
    pub data: String,
}
