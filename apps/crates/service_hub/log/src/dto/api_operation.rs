//! API操作日志

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::log::{log_api_operation, log_api_operation::enums::HttpType};

/// 查询API操作日志列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetApiOperationsReq {
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
pub struct GetApiOperationsResp {
    pub data_list: Vec<log_api_operation::Model>,
    pub total: u64,
}

impl From<(Vec<log_api_operation::Model>, u64)> for GetApiOperationsResp {
    fn from((data_list, total): (Vec<log_api_operation::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetApiOperationReq {
    /// 字典数据ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetApiOperationResp {
    #[serde(flatten)]
    model: log_api_operation::Model,
}

impl From<log_api_operation::Model> for GetApiOperationResp {
    fn from(model: log_api_operation::Model) -> Self {
        Self { model }
    }
}

/// 添加API操作日志 请求体
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateApiOperationReq {
    /// 用户ID
    pub user_id: Option<i32>,
    /// 用户名称
    pub username: Option<String>,
    /// 请求ID
    pub request_id: Option<String>,
    /// 请求状态码
    pub status_code: i32,
    /// 请求方法
    pub method: String,
    /// 请求地址路径
    pub path: String,
    /// Content-Type
    pub content_type: String,
    /// 请求参数
    pub query: Option<String>,
    /// 请求体/响应体
    pub body: Option<String>,
    /// 请求IP
    pub remote_addr: String,
    /// 用户代理
    pub user_agent: String,
    /// 耗时,纳秒
    pub cost: i16,
    /// 请求类型:REQ/RESP
    pub http_type: HttpType,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiOperationResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteApiOperationReq {
    /// 字典数据ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteApiOperationResp {}
