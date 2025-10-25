//! 模板管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::template::t_app_template;

/// 查询列表数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetAppTemplatesReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 排序字段
    pub order_by: Option<String>,
    /// 返回有所有数据
    pub is_all: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAppTemplatesResp {
    pub data_list: Vec<t_app_template::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetAppTemplateReq {
    /// 模板ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAppTemplateResp {
    #[serde(flatten)]
    data: t_app_template::Model,
}

/// 添加数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct CreateAppTemplateReq {
    /// 用户ID
    pub user_id: i32,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppTemplateResp {}

/// 批量添加数据结点
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchCreateAppTemplateItem {
    /// 用户ID
    pub user_id: i32,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 批量添加数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct BatchCreateAppTemplateReq {
    /// 数据列表
    pub data: Vec<BatchCreateAppTemplateItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateAppTemplateResp {}

/// 更新数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct UpdateAppTemplateReq {
    /// 模板ID
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppTemplateResp {}

/// 更新数据状态 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct UpdateAppTemplateStatusReq {
    /// 模板ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppTemplateStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteAppTemplateReq {
    /// 模板ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAppTemplateResp {}

/// 批量删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct BatchDeleteAppTemplateReq {
    /// 模板ID列表
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteAppTemplateResp {}
