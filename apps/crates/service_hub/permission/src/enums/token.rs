//! 令牌表
use serde::{Deserialize, Serialize};

/// 令牌权限范围
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    /// 读取数据
    #[serde(rename = "GET")]
    GET,
    /// 提交数据
    #[serde(rename = "POST")]
    POST,
    /// 更新数据
    #[serde(rename = "PUT")]
    PUT,
    /// 删除数据
    #[serde(rename = "DELETE")]
    DELETE,
}
