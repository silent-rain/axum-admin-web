//! 令牌管理

use entity::permission::token;

use utils::time::{default_naive_date_time, str_to_naive_date_time};

use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 查询令牌列表
#[derive(Default, Deserialize, Validate)]
pub struct GetTokensReq {
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
    /// 令牌
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokensResp {
    pub data_list: Vec<token::Model>,
    pub total: u64,
}

/// 查询令牌详情 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetTokenReq {
    /// 令牌ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenResp {
    #[serde(flatten)]
    data: token::Model,
}

/// 添加令牌
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateTokenReq {
    /// 用户ID
    pub user_id: i32,
    /// 权限范围:GET,POST,PUT,DELETE
    /// Enum: [`token::enums::Permission`]
    pub permission: String,
    /// 授权到期时间
    #[serde(
        rename = "expire",
        deserialize_with = "str_to_naive_date_time",
        default = "default_naive_date_time"
    )]
    pub expire: DateTime,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTokenResp {}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTokenReq {
    /// 令牌ID
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 权限范围:GET,POST,PUT,DELETE
    /// Enum: [`crate::enums::TokenPermission`]
    pub permission: String,
    /// 授权到期时间
    #[serde(
        rename = "expire",
        deserialize_with = "str_to_naive_date_time",
        default = "default_naive_date_time"
    )]
    pub expire: DateTime,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTokenResp {}

/// 更新令牌状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTokenStatusReq {
    /// 令牌ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTokenStatusResp {}

/// 删除令牌 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteTokenReq {
    /// 令牌ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTokenResp {}
