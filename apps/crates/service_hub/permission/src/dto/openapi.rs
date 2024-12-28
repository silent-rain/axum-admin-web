//! OpenApi接口管理

use entity::permission::openapi;

use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 查询OpenApi接口列表 请求体
#[derive(Clone, Deserialize, Validate)]
pub struct GetOpenapisReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 接口名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenapisResp {
    pub data_list: Vec<openapi::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetOpenapiReq {
    /// 接口ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenapiResp {
    #[serde(flatten)]
    data: openapi::Model,
}

/// 添加OpenApi接口
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct CreateOpenapiReq {
    /// 父ID
    pub pid: Option<i32>,
    /// 类别,0:目录,1:接口
    pub category: openapi::enums::Category,
    /// 接口名称
    pub name: String,
    /// 请求类型
    pub method: String,
    /// 资源路径
    pub path: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态, 0:停用,1:正常
    pub status: openapi::enums::Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOpenapiResp {}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateOpenapiReq {
    /// 接口ID
    pub id: i32,
    /// 父ID
    pub pid: Option<i32>,
    /// 类别,0:目录,1:接口
    pub category: openapi::enums::Category,
    /// 接口名称
    pub name: String,
    /// 请求类型
    pub method: String,
    /// 资源路径
    pub path: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
    pub status: openapi::enums::Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOpenapiResp {}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateOpenapiStatusReq {
    /// 接口ID
    pub id: i32,
    /// 状态(0:停用,1:正常)
    pub status: openapi::enums::Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOpenapiStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteOpenapiReq {
    /// 接口ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOpenapiResp {}

/// 角色接口关系权限
#[derive(Clone, Serialize, Deserialize, Validate, FromQueryResult)]
pub struct RoleOpenapiPermission {
    /// 角色ID
    pub role_id: i32,
    /// 请求类型
    pub method: String,
    /// 资源路径
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenapiTreeItem {
    #[serde(flatten)]
    pub data: openapi::Model,
    pub children: Vec<OpenapiTreeItem>,
}

/// 菜单数列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct GetOpenapiTreeReq {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenapiTreeResp {
    #[serde(flatten)]
    pub data: OpenapiTreeItem,
}
