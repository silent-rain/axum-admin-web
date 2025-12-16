//! 任务调度作业管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::schedule::schedule_job;

use crate::enums::schedule_job::{JobType, Source, Status};

/// 查询任务调度列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetScheduleJobsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 任务名称
    pub name: Option<String>,
    /// 任务类型
    pub job_type: Option<i8>,
    /// 任务状态
    pub status: Option<i8>,
}

/// 查询任务调度列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetScheduleJobsResp {
    pub data_list: Vec<schedule_job::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetScheduleJobReq {
    /// 任务调度ID
    pub id: i32,
}

/// 查询数据 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetScheduleJobResp {
    #[serde(flatten)]
    model: schedule_job::Model,
}

/// 添加任务调度 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateScheduleJobReq {
    /// 任务名称
    pub name: String,
    /// 任务来源(0:用户定义,1:系统内部)
    pub source: Source,
    /// 任务类型,0:任务调度,1:即时任务
    pub job_type: JobType,
    /// 系统任务编码
    pub sys_code: Option<String>,
    /// cron表达式
    pub expression: Option<String>,
    /// 间隔时间,秒
    pub interval: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
}

/// 更新数据 请求体
#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateScheduleJobReq {
    /// 任务调度ID
    pub id: i32,
    /// 任务名称
    pub name: String,
    /// cron表达式
    pub expression: Option<String>,
    /// 间隔时间,秒
    pub interval: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
}

/// 更新数据状态 请求体
#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateScheduleJobStatusReq {
    /// 任务调度ID
    pub id: i32,
    /// 任务状态(0:下线,1:上线)
    pub status: Status,
}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteScheduleJobReq {
    /// 任务调度ID
    pub id: i32,
}
