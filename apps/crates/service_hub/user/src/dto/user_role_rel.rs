//! 用户角色关系管理
use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::user::user_role_rel;

/// 查询用户角色关系列表
#[derive(Default, Deserialize, Validate)]
pub struct GetUserRoleRelsReq {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRoleRelsResp {
    pub data_list: Vec<user_role_rel::Model>,
    pub total: u64,
}

impl From<(Vec<user_role_rel::Model>, u64)> for GetUserRoleRelsResp {
    fn from((data_list, total): (Vec<user_role_rel::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 批量添加用户角色关系
#[derive(Serialize, Deserialize, Validate)]
pub struct BatchCreateUserRoleRelReq {
    /// 用户ID
    pub user_id: i32,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateUserRoleRelResp {}

/// 批量删除用户角色关系
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteUserRoleRelReq {
    /// ID列表
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteUserRoleRelResp {}
