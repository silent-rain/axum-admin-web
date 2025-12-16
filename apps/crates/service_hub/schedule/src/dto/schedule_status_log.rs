//! 任务调度状态日志管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::schedule::schedule_status_log;

use crate::enums::schedule_job::Status;

/// 查询任务调度状态日志列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetScheduleStatusLogsReq {
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
    /// 任务状态
    pub status: Option<i8>,
}

/// 查询任务调度状态日志列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetScheduleStatusLogsResp {
    pub data_list: Vec<schedule_status_log::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetScheduleStatusLogReq {
    /// 状态日志ID
    pub id: i32,
}

/// 查询数据 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetScheduleStatusLogResp {
    #[serde(flatten)]
    model: schedule_status_log::Model,
}

/// 添加任务调度状态日志 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateScheduleStatusLogReq {
    /// 任务ID
    pub job_id: i32,
    /// 任务调度ID
    pub uuid: String,
}

/// 更新任务调度状态日志 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateScheduleStatusLogReq {
    /// 状态日志ID
    pub id: i32,
    /// 任务ID
    pub job_id: i32,
    /// 任务调度ID
    pub uuid: String,
    /// 失败信息
    pub error: Option<String>,
    /// 耗时,毫秒
    pub cost: u64,
    /// 任务状态,0:失败,1:成功
    pub status: Status,
}

/// 更新数据状态 请求体
#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateScheduleStatusLogSatausReq {
    /// 状态日志ID
    pub id: i32,
    /// 任务状态,0:失败,1:成功
    pub status: Status,
}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteScheduleStatusLogReq {
    /// 状态日志ID
    pub id: i32,
}
