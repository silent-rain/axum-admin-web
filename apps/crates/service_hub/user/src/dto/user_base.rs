//! 用户信息管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::user::{role, user_base};

use crate::enums::user_base::Gender;

/// 查询用户列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetUserBasesReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户名称
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserBasesResp {
    pub data_list: Vec<user_base::Model>,
    pub total: u64,
}

impl From<(Vec<user_base::Model>, u64)> for GetUserBasesResp {
    fn from((data_list, total): (Vec<user_base::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 查询用户信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetUserBaseReq {
    /// 用户ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserBaseResp {
    #[serde(flatten)]
    model: user_base::Model,
}

impl From<user_base::Model> for GetUserBaseResp {
    fn from(model: user_base::Model) -> Self {
        Self { model }
    }
}

/// 添加用户 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserBaseReq {
    /// 用户名称
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别(0:保密,1:女,2:男)
    pub gender: Gender,
    /// 密码
    pub password: String,
    /// 状态(false:停用,true:正常)
    pub status: bool,
    /// 年龄
    pub age: Option<i32>,
    /// 出生日期
    pub date_birth: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 用户个人介绍
    pub intro: Option<String>,
    /// 用户描述
    pub desc: Option<String>,
    /// 用户的居住或邮寄地址
    pub address: Option<String>,
    /// 偏好设置
    pub preferences: Option<String>,
    /// 所属部门ID
    pub department_id: Option<i32>,
    /// 所属岗位ID
    pub position_id: Option<i32>,
    /// 所属职级ID
    pub rank_id: Option<i32>,
    /// 用户会员等级ID
    pub member_level_id: Option<i32>,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserBaseResp {}

/// 更新用户 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserBaseReq {
    /// 用户ID
    pub id: i32,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别(0:保密,1:女,2:男)
    pub gender: Gender,
    /// 状态(false:停用,true:正常)
    pub status: bool,
    /// 年龄
    pub age: Option<i32>,
    /// 出生日期
    pub date_birth: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 用户个人介绍
    pub intro: Option<String>,
    /// 用户描述
    pub desc: Option<String>,
    /// 用户的居住或邮寄地址
    pub address: Option<String>,
    /// 偏好设置
    pub preferences: Option<String>,
    /// 所属部门ID
    pub department_id: Option<i32>,
    /// 所属岗位ID
    pub position_id: Option<i32>,
    /// 所属职级ID
    pub rank_id: Option<i32>,
    /// 用户会员等级ID
    pub member_level_id: Option<i32>,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserBaseResp {}

/// 更新用户状态 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserBaseStatusReq {
    /// 用户ID
    pub id: i32,
    /// 用户状态
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserBaseStatusResp {}

/// 删除用户 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteUserBaseReq {
    /// 用户ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserBaseResp {}

/// 更新用户分享码 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct ProfileReq {
    /// 用户ID
    pub id: i32,
}

/// 获取用户个人信息
#[derive(Clone, Serialize, Deserialize)]
pub struct ProfileResp {
    /// 用户ID
    pub id: i32,
    /// 用户名称
    pub username: String,
    /// 性别
    pub gender: i8,
    /// 年龄
    pub age: Option<i32>,
    /// 出生日期
    pub date_birth: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
}

/// 通过用户信息ID获角色色列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct RolesReq {
    /// 用户ID
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolesResp {
    pub data_list: Vec<role::Model>,
    pub total: u64,
}

impl From<(Vec<role::Model>, u64)> for RolesResp {
    fn from((data_list, total): (Vec<role::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 用户接口权限权限
#[derive(Clone, Serialize, Deserialize)]
pub struct UserPermission {
    pub user_id: i32,
    pub username: String,
    pub role_ids: Vec<i32>,
}

/// 查询用户信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetCheckUsernameReq {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCheckUsernameResp {}
