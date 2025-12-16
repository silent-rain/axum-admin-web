//! OpenApi接口管理

use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::enums::openapi::Category;
use database::utils::GenericTree;
use entity::permission::openapi;

/// 查询OpenApi接口列表 请求体
#[derive(Deserialize, Validate)]
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

/// 查询OpenApi接口列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenapisResp {
    pub data_list: Vec<openapi::Model>,
    pub total: u64,
}

/// 查询OpenApi接口详情 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetOpenapiReq {
    /// 接口ID
    pub id: i32,
}

/// 查询OpenApi接口详情 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenapiResp {
    #[serde(flatten)]
    model: openapi::Model,
}

/// 添加OpenApi接口 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateOpenapiReq {
    /// 父ID
    pub pid: Option<i32>,
    /// 类别,0:目录,1:接口
    pub category: Category,
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
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新数据 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateOpenapiReq {
    /// 接口ID
    pub id: i32,
    /// 父ID
    pub pid: Option<i32>,
    /// 类别,0:目录,1:接口
    pub category: Category,
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
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 更新数据状态 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateOpenapiStatusReq {
    /// 接口ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteOpenapiReq {
    /// 接口ID
    pub id: i32,
}

/// 角色接口关系权限 响应体
#[derive(Serialize, Deserialize, Validate, FromQueryResult)]
pub struct RoleOpenapiPermission {
    /// 角色ID
    pub role_id: i32,
    /// 请求类型
    pub method: String,
    /// 资源路径
    pub path: String,
}

/// 接口树列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct GetOpenapiTreeReq {}

/// OpenApi接口树列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenapiTreeResp {
    data_list: Vec<GenericTree<openapi::Model>>,
}
