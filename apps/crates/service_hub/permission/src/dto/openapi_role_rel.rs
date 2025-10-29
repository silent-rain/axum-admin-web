//! OpenApi接口角色关系管理
use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::permission::openapi_role_rel;

/// 查询OpenApi接口角色关系列表
#[derive(Default, Deserialize, Validate)]
pub struct GetOpenapiRoleRelsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 接口ID
    pub openapi_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenapiRoleRelsResp {
    pub data_list: Vec<openapi_role_rel::Model>,
    pub total: u64,
}

impl From<(Vec<openapi_role_rel::Model>, u64)> for GetOpenapiRoleRelsResp {
    fn from((data_list, total): (Vec<openapi_role_rel::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 批量添加OpenApi接口角色关系
#[derive(Serialize, Deserialize, Validate)]
pub struct BatchCreateOpenapiRoleRelReq {
    /// 接口ID
    pub openapi_id: i32,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateOpenapiRoleRelResp {}

/// 批量删除OpenApi接口角色关系
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteOpenapiRoleRelReq {
    /// ID列表
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteOpenapiRoleRelResp {}
