//! WEB日志管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::log::{
    log_web,
    log_web::enums::{ErrorType, OsType},
};

/// 查询WEB日志列表 请求体
#[derive(Default, Deserialize, Serialize, Validate)]
pub struct GetWebLogsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 用户名称
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWebLogsResp {
    pub data_list: Vec<log_web::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetWebLogReq {
    /// 日志ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWebLogResp {
    #[serde(flatten)]
    data: log_web::Model,
}

/// 添加WEB日志信息 请求体
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateWebLogReq {
    /// 用户ID
    pub user_id: Option<i32>,
    /// 用户名称
    pub username: Option<String>,
    /// 请求ID
    pub request_id: Option<String>,
    /// 终端类型
    pub os_type: OsType,
    /// 错误类型
    pub error_type: ErrorType,
    /// 日志级别
    pub level: String,
    /// 日发生位置
    pub caller_line: String,
    /// 错误页面
    pub url: Option<String>,
    /// 日志消息
    pub msg: Option<String>,
    /// 堆栈信息
    pub stack: Option<String>,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWebLogResp {}
