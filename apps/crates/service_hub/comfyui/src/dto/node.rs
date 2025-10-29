//! ComfyUI 节点管理

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 获取节点信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct ObjectInfoReq {
    pub node_class: Option<String>,
}

/// 获取扩展节点列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExtensionsResp {
    pub data_list: Vec<String>,
    pub total: u64,
}

impl From<(Vec<String>, u64)> for ExtensionsResp {
    fn from((data_list, total): (Vec<String>, u64)) -> Self {
        Self { data_list, total }
    }
}
