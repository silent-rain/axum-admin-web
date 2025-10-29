//! 角色管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::user::role;

/// 查询角色列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetRolesReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 角色名称
    pub name: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRolesResp {
    pub data_list: Vec<role::Model>,
    pub total: u64,
}

impl From<(Vec<role::Model>, u64)> for GetRolesResp {
    fn from((data_list, total): (Vec<role::Model>, u64)) -> Self {
        Self { data_list, total }
    }
}

/// 查询角色信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetRoleReq {
    /// 角色ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRoleResp {
    #[serde(flatten)]
    model: role::Model,
}

impl From<role::Model> for GetRoleResp {
    fn from(model: role::Model) -> Self {
        Self { model }
    }
}

/// 添加角色 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateRoleReq {
    /// 角色名称
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
pub struct CreateRoleResp {}

/// 更新角色信息 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateRoleReq {
    /// 角色ID
    pub id: i32,
    /// 角色名称
    pub name: String,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleResp {}

/// 更新角色状态 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateRoleStatusReq {
    /// 角色ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleStatusResp {}

/// 删除角色 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteRoleReq {
    /// 角色ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRoleResp {}

#[cfg(test)]
mod tests {
    use super::*;
    use err_code::Error;

    use serde_json::json;

    #[test]
    fn test_status() -> Result<(), Error> {
        let expected = UpdateRoleStatusReq {
            id: 1,
            status: true,
        };
        let json_data = json!({ "id": 1, "status":true });
        let result: UpdateRoleStatusReq = serde_json::from_value(json_data)
            .map_err(|err| Error::JsonDeserialization(err.to_string()))?;
        assert!(expected == result);

        Ok(())
    }
}
