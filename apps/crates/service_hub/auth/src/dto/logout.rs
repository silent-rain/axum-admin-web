//! 登出

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 登出 请求体
#[derive(Default, Clone, Deserialize, Validate)]
pub struct LogoutReq {}

/// 登陆 响应体
#[derive(Default, Deserialize, Serialize)]
pub struct LogoutResp {}
