//! 任务调度事件日志管理

use entity::schedule::schedule_event_log;

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 查询任务调度事件日志列表 请求体
#[derive(Default, Deserialize)]
pub struct GetScheduleEventLogsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 任务ID
    pub job_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetScheduleEventLogsResp {
    pub data_list: Vec<schedule_event_log::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetScheduleEventLogReq {
    /// 事件日志ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetScheduleEventLogResp {
    #[serde(flatten)]
    data: schedule_event_log::Model,
}

/// 添加任务调度事件日志 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct CreateScheduleEventLogReq {
    /// 任务ID
    pub job_id: i32,
    /// 任务调度ID
    pub uuid: String,
    /// 任务状态,0:失败,1:成功'
    pub status: schedule_event_log::enums::Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScheduleEventLogResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteScheduleEventLogReq {
    /// 事件日志ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteScheduleEventLogResp {}
