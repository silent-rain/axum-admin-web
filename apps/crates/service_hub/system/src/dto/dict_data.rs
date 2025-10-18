//! 字典数据管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::system::dict_data;

/// 查询字典数据列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetDictDatasReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 字典项标签
    pub lable: Option<String>,
    /// 字典维度ID
    pub dim_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDictDatasResp {
    pub data_list: Vec<dict_data::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetDictDataReq {
    /// 字典数据ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDictDataResp {
    #[serde(flatten)]
    data: dict_data::Model,
}

/// 添加字典数据 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateDictDataReq {
    /// 字典维度ID
    pub dim_id: i32,
    /// 字典项标签
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub lable: String,
    /// 字典项值
    pub value: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDictDataResp {}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDictDataReq {
    /// 字典数据ID
    pub id: i32,
    /// 字典项标签
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub lable: String,
    /// 字典项值
    pub value: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDictDataResp {}

/// 更新字典数据状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDictDataStatusReq {
    /// 字典数据ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDictDataStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDictDataReq {
    /// 字典数据ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDictDataResp {}
