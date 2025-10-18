//! 登陆日志管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::user::{user_login_log, user_login_log::enums::LoginStatus};

/// 查询登陆日志列表 请求体
#[derive(Default, Deserialize, Serialize, Validate)]
pub struct GetUserLoginLogsReq {
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
pub struct GetUserLoginLogsResp {
    pub data_list: Vec<user_login_log::Model>,
    pub total: u64,
}

/// 查询登陆日志信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetUserLoginLogReq {
    /// 钱包ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserLoginLogResp {
    #[serde(flatten)]
    data: user_login_log::Model,
}

/// 添加登陆日志信息 请求体
#[derive(Deserialize, Validate)]
pub struct CreateUserLoginLogReq {
    /// 用户ID
    pub user_id: i32,
    /// 用户名称
    pub username: String,
    /// 用户会话ID
    pub session_id: String,
    /// 登录IP
    pub remote_addr: String,
    /// 用户代理
    pub user_agent: String,
    /// 描述信息
    pub desc: Option<String>,
    /// 登录状态
    pub login_status: LoginStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserLoginLogResp {}
