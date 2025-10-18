//! 字典维度管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::system::dict_dimension;

/// 查询字典维度列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetDictDimensionsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 字典维度名称
    pub name: Option<String>,
    /// 字典维度编码
    pub code: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDictDimensionsResp {
    pub data_list: Vec<dict_dimension::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetDictDimensionReq {
    /// 字典维度ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDictDimensionResp {
    #[serde(flatten)]
    data: dict_dimension::Model,
}

/// 添加字典维度 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateDictDimensionReq {
    /// 字典维度名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 字典维度编码
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub code: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDictDimensionResp {}

/// 更新字典维度 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDictDimensionReq {
    /// 字典维度ID
    pub id: i32,
    /// 字典维度名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 字典维度编码
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub code: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDictDimensionResp {}

/// 更新字典维度状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDictDimensionStatusReq {
    /// 字典维度ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDictDimensionStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDictDimensionReq {
    /// 字典维度ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDictDimensionResp {}
