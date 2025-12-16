//! 部门角色关系管理
use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::organization::department_role_rel;

/// 查询部门角色关系列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetDepartmentRoleRelsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 部门ID
    pub department_id: Option<i32>,
}

/// 查询部门角色关系列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDepartmentRoleRelsResp {
    pub data_list: Vec<department_role_rel::Model>,
    pub total: u64,
}

/// 批量添加部门角色关系 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct BatchCreateDepartmentRoleRelReq {
    /// 部门ID
    pub department_id: i32,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

/// 批量删除部门角色关系 请求体
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteDepartmentRoleRelReq {
    /// ID列表
    pub ids: Vec<i32>,
}
