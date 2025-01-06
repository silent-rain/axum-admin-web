//! 测试

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Say Hello 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct SayHelloReq {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SayHelloResp {
    pub msg: String,
}

/// Say Hi 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct SayHiReq {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SayHiResp {
    pub msg: String,
}
