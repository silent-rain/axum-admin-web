//! 用户session管理

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use validator::Validate;

use entity::user::user_session;

/// 查询用户session列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetUserSessionsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户会话ID
    pub session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserSessionsResp {
    pub data_list: Vec<user_session::Model>,
    pub total: u64,
}

/// 查询用户session信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetUserSessionReq {
    /// session ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserSessionResp {
    #[serde(flatten)]
    data: user_session::Model,
}

/// 添加用户session 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserSessionReq {
    /// 用户会话ID
    pub session_id: String,
    /// 过期时间
    pub expiry_date: OffsetDateTime,
    /// 元数据
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserSessionResp {}

/// 更新用户session 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateUserSessionReq {
    /// session ID
    pub id: i32,
    /// 过期时间
    pub expiry_date: OffsetDateTime,
    /// 元数据
    pub data: Vec<u8>,
    /// 用户session状态
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserSessionResp {}

/// 更新用户session状态 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserSessionStatusReq {
    /// 用户ID
    pub id: i32,
    /// 用户session状态
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserSessionStatusResp {}

/// 删除用户session 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteUserSessionReq {
    /// session ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserSessionResp {}
