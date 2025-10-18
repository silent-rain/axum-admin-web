//! 用户邮箱管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::user::email;

/// 查询用户列表
#[derive(Default, Deserialize, Validate)]
pub struct GetEmailsReq {
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
    /// 邮箱
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEmailsResp {
    pub data_list: Vec<email::Model>,
    pub total: u64,
}

/// 查询邮箱信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetEmailReq {
    /// 邮箱ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEmailResp {
    #[serde(flatten)]
    data: email::Model,
}

/// 添加邮箱 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateEmailReq {
    /// 用户ID
    pub user_id: i32,
    /// 邮箱
    #[validate(email)]
    pub email: String,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmailResp {}

/// 更新邮箱 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateEmailReq {
    /// 邮箱ID
    pub id: i32,
    /// 邮箱
    #[validate(email)]
    pub email: String,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEmailResp {}

/// 删除邮箱 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteEmailReq {
    /// 邮箱ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteEmailResp {}
