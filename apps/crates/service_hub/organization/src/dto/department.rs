//! 部门管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entity::department;

/// 查询部门列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetDepartmentsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 部门名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDepartmentsResp {
    pub data_list: Vec<department::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetDepartmentReq {
    /// 模板ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDepartmentResp {
    #[serde(flatten)]
    data: department::Model,
}

/// 添加部门 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateDepartmentReq {
    /// 上级部门ID
    pub pid: Option<i32>,
    /// 所有上级部门ID, 用逗号分开
    pub pids: Option<String>,
    /// 部门名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDepartmentResp {}

/// 更新数据 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDepartmentReq {
    /// 部门ID
    pub id: i32,
    /// 上级部门ID
    pub pid: Option<i32>,
    /// 所有上级部门ID, 用逗号分开
    pub pids: Option<String>,
    /// 部门名称
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDepartmentResp {}

/// 更新数据状态 请求体
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDepartmentStatusReq {
    /// 部门ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDepartmentStatusResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteDepartmentReq {
    /// 部门ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDepartmentResp {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentTreeItem {
    #[serde(flatten)]
    pub data: department::Model,
    pub children: Vec<DepartmentTreeItem>,
}

/// 部门树列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct GetDepartmentTreeReq {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDepartmentTreeResp {
    #[serde(flatten)]
    pub data: DepartmentTreeItem,
}
