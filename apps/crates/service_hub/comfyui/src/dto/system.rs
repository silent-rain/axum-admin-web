//! ComfyUI 系统管理

use serde::{Deserialize, Serialize};

use crate::api_clients::dto::SystemStats;

/// 获取所有的队列 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SystemStatsResp {
    #[serde(flatten)]
    pub data: SystemStats,
}
