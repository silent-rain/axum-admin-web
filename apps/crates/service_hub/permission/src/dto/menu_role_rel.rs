//! 菜单角色关系管理
use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::permission::menu_role_rel;

/// 查询菜单角色关系列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetMenuRoleRelsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 菜单ID
    pub menu_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMenuRoleRelsResp {
    pub data_list: Vec<menu_role_rel::Model>,
    pub total: u64,
}

impl From<(Vec<menu_role_rel::Model>, u64)> for GetMenuRoleRelsResp {
    fn from((data_list, total): (Vec<menu_role_rel::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 批量添加菜单角色关系 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct BatchCreateMenuRoleRelReq {
    /// 菜单ID
    pub menu_id: i32,
    /// 角色ID列表
    pub role_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateMenuRoleRelResp {}

/// 批量删除菜单角色关系 请求体
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteMenuRoleRelReq {
    /// ID列表
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteMenuRoleRelResp {}
