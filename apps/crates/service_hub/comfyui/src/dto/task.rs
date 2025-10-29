//! ComfyUI 任务管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_clients::dto::{Image, Queues};
pub use crate::api_clients::dto::{PromptReq as PushPromptReq, PromptResult, QueueRemaining};

/// 发布绘图任务 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PushPromptResp {
    #[serde(flatten)]
    pub data: PromptResult,
}

impl From<PromptResult> for PushPromptResp {
    fn from(data: PromptResult) -> Self {
        Self { data }
    }
}

/// 获取服务器当前剩余任务列队的数量 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueueRemainingResp {
    #[serde(flatten)]
    pub data: QueueRemaining,
}

impl From<QueueRemaining> for QueueRemainingResp {
    fn from(data: QueueRemaining) -> Self {
        Self { data }
    }
}

/// 获取所有历史任务数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct HistorysReq {
    pub max_items: Option<i32>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct History {
    pub prompt_id: String,
    pub images: Vec<Image>,
}

/// 获取所有历史任务数据 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HistorysResp {
    pub data_list: Vec<History>,
    pub total: u64,
}

impl From<(Vec<History>, u64)> for HistorysResp {
    fn from((data_list, total): (Vec<History>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 获取指定历史任务数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct HistoryReq {
    pub prompt_id: String,
}

/// 获取指定历史任务数据 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HistoryResp {
    #[serde(flatten)]
    pub data: History,
}

impl From<History> for HistoryResp {
    fn from(data: History) -> Self {
        Self { data }
    }
}

/// 获取所有的队列 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueuesResp {
    #[serde(flatten)]
    pub data: Queues,
}

impl From<Queues> for QueuesResp {
    fn from(data: Queues) -> Self {
        Self { data }
    }
}

/// 删除队列 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct DeleteQueueReq {
    pub prompt_ids: Vec<String>,
}
