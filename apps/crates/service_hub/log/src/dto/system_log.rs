//! 系统日志

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::log::log_system;

/// 查询系统日志列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetSystemLogsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSystemLogsResp {
    pub data_list: Vec<log_system::Model>,
    pub total: u64,
}

impl From<(Vec<log_system::Model>, u64)> for GetSystemLogsResp {
    fn from((data_list, total): (Vec<log_system::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetSystemLogReq {
    /// 日志ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSystemLogResp {
    #[serde(flatten)]
    model: log_system::Model,
}

impl From<log_system::Model> for GetSystemLogResp {
    fn from(model: log_system::Model) -> Self {
        Self { model }
    }
}

/// 添加系统日志 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateSystemLogReq {
    /// 用户ID
    pub user_id: Option<i32>,
    /// 用户名称
    pub username: Option<String>,

    /// 日志记录器名称
    #[serde(default)]
    pub name: String,
    /// Parent Span Id
    #[serde(default)]
    pub span_pid: Option<u32>,
    /// Span Id
    #[serde(default)]
    pub span_id: Option<u32>,
    /// 模块路径
    #[serde(default)]
    pub module_path: Option<String>,
    /// 描述发生此元数据所描述的跨度或事件的系统部分
    #[serde(default)]
    pub target: String,
    /// 文件
    #[serde(default)]
    pub file: Option<String>,
    /// 报错行数
    #[serde(default)]
    pub line: Option<u32>,
    /// 日志级别
    #[serde(default)]
    pub level: String,
    /// 事件类型
    #[serde(default)]
    pub kind: String,
    /// 是否为事件
    #[serde(default)]
    pub is_event: bool,
    /// 是否为 span
    #[serde(default)]
    pub is_span: bool,
    /// 日志字段名称列表
    #[serde(default)]
    pub fields: Option<String>,
    /// fields 日志数据集
    pub field_data: Option<String>,
    /// 日志信息
    pub message: Option<String>,

    /// 业务码
    pub code: Option<i32>,
    /// 业务码信息
    pub code_msg: Option<String>,
    /// 堆栈信息
    pub stack: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSystemLogResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteSystemLogReq {
    /// 日志ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSystemLogResp {}
