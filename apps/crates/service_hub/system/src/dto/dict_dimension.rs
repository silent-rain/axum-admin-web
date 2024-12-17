//! 字典维度管理

use entity::system::sys_dict_dimension;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询字典维度列表
#[derive(Default, Deserialize, Validate)]
pub struct GetDictDimensionListReq {
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

/// 添加字典维度
#[derive(Serialize, Deserialize, Validate)]
pub struct AddDictDimensionReq {
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

/// 更新字典维度 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDictDimensionReq {
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
    /// 状态(0:停用,1:正常)
    pub status: sys_dict_dimension::enums::Status,
}

/// 更新字典维度状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateDictDimensionStatusReq {
    /// 状态(0:停用,1:正常)
    pub status: sys_dict_dimension::enums::Status,
}
