//! 配置管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use database::utils::GenericTree;
use entity::system::config;

/// 查询配置列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetConfigsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 配置名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConfigsResp {
    pub data_list: Vec<config::Model>,
    pub total: u64,
}

impl From<(Vec<config::Model>, u64)> for GetConfigsResp {
    fn from((data_list, total): (Vec<config::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetConfigReq {
    /// 配置ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConfigResp {
    #[serde(flatten)]
    model: config::Model,
}

impl From<config::Model> for GetConfigResp {
    fn from(model: config::Model) -> Self {
        Self { model }
    }
}

/// 添加配置 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateConfigReq {
    /// 父节点ID
    pub pid: Option<i32>,
    /// 配置名称
    pub name: String,
    /// 配置编码(英文)
    pub code: String,
    /// 配置值
    pub value: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 配置描述
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConfigResp {}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateConfigReq {
    /// 配置ID
    pub id: i32,
    /// 父节点ID
    pub pid: Option<i32>,
    /// 配置名称
    pub name: String,
    /// 配置编码(英文)
    pub code: String,
    /// 配置值
    pub value: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 配置描述
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConfigResp {}

/// 更新数据状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateConfigStatusReq {
    /// 配置ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConfigStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteConfigReq {
    /// 配置ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteConfigResp {}

/// 配置树列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct GetConfigTreeReq {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConfigTreeResp {
    #[serde(flatten)]
    pub data: Vec<GenericTree<config::Model>>,
}

impl From<Vec<GenericTree<config::Model>>> for GetConfigTreeResp {
    fn from(data: Vec<GenericTree<config::Model>>) -> Self {
        Self { data }
    }
}
