//! 会员等级管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::user::member_level;

/// 查询会员等级列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetMemberLevelsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 会员等级名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMemberLevelsResp {
    pub data_list: Vec<member_level::Model>,
    pub total: u64,
}

/// 查询会员等级信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetMemberLevelReq {
    /// 会员等级ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMemberLevelResp {
    #[serde(flatten)]
    data: member_level::Model,
}

/// 添加会员等级 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateMemberLevelReq {
    /// 会员等级名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    ///会员等级等级
    pub level: u16,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMemberLevelResp {}

/// 更新会员等级信息 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMemberLevelReq {
    /// 会员等级ID
    pub id: i32,
    /// 会员等级名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    ///会员等级等级
    pub level: u16,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMemberLevelResp {}

/// 更新会员等级状态 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMemberLevelStatusReq {
    /// 会员等级ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMemberLevelStatusResp {}

/// 删除会员等级 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteMemberLevelReq {
    /// 钱包ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMemberLevelResp {}
