//! 岗位管理

use entity::organization::position;

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 查询岗位列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetPositionsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 岗位名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPositionsResp {
    pub data_list: Vec<position::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetPositionReq {
    /// 岗位ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPositionResp {
    #[serde(flatten)]
    data: position::Model,
}

/// 添加岗位 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreatePositionReq {
    /// 岗位名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 所属部门ID
    pub department_id: Option<i32>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePositionResp {}

/// 更新数据 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePositionReq {
    /// 岗位ID
    pub id: i32,
    /// 岗位名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 所属部门ID
    pub department_id: Option<i32>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePositionResp {}

/// 更新数据状态 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePositionStatusReq {
    /// 岗位ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePositionStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeletePositionReq {
    /// 岗位ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePositionResp {}
