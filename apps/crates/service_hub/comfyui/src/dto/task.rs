//! ComfyUI 任务管理

use serde::{Deserialize, Serialize};
use validator::Validate;

pub use crate::api_clients::dto::{
    PromptReq as PushPromptReq, PromptResult as PushPromptResp,
    QueueRemaining as QueueRemainingResp,
};

/// 绘图任务的下发接口 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct HistoryReq {
    pub prompt_id: Option<String>,
}

/// 获取所有的队列 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueuesResp {
    pub prompt_id: Option<String>,
}

/// 绘图任务的下发接口 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HistoryResp {}

/// 删除队列 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct DeleteQueueReq {
    pub prompt_ids: Vec<String>,
}
