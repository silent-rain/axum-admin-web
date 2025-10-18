//! 职级管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::organization::rank;

/// 查询职级列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetRanksReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 职级名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRanksResp {
    pub data_list: Vec<rank::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetRankReq {
    /// 职级ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRankResp {
    #[serde(flatten)]
    data: rank::Model,
}

/// 添加职级 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateRankReq {
    /// 职级名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    ///职级等级
    pub level: u16,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRankResp {}

/// 更新数据 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateRankReq {
    /// 职级ID
    pub id: i32,
    /// 职级名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    ///职级等级
    pub level: u16,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRankResp {}

/// 更新数据状态 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateRankStatusReq {
    /// 职级ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRankStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteRankReq {
    /// 职级ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRankResp {}
