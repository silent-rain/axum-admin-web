//! 用户角色关系管理

use log::error;
use nject::injectable;
use sea_orm::Set;

use entity::user::user_role_rel;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::user_role_rel::UserRoleRelDao,
    dto::user_role_rel::{BatchCreateUserRoleRelReq, GetUserRoleRelsReq},
};

/// 服务层
#[injectable]
pub struct UserRoleRelService {
    user_role_rel_dao: UserRoleRelDao,
}

impl UserRoleRelService {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<user_role_rel::Model>, u64), ErrorMsg> {
        let (results, total) = self.user_role_rel_dao.all().await.map_err(|err| {
            error!("查询所有用户角色关系列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询所有用户角色关系列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetUserRoleRelsReq,
    ) -> Result<(Vec<user_role_rel::Model>, u64), ErrorMsg> {
        let (results, total) = self.user_role_rel_dao.list(req).await.map_err(|err| {
            error!("查询用户角色关系列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询用户角色关系列表失败")
        })?;

        Ok((results, total))
    }

    /// 批量添加数据
    pub async fn batch_create(&self, req: BatchCreateUserRoleRelReq) -> Result<i32, ErrorMsg> {
        let mut models = Vec::new();
        for role_id in req.role_ids {
            let model = user_role_rel::ActiveModel {
                user_id: Set(req.user_id),
                role_id: Set(role_id),
                ..Default::default()
            };
            models.push(model);
        }

        let result = self
            .user_role_rel_dao
            .batch_create(models)
            .await
            .map_err(|err| {
                error!("批量添加用户角色关系失败, err: {:#?}", err);
                Error::DbBatchAddError.into_err_with_msg("批量添加用户角色关系失败")
            })?;

        Ok(result)
    }

    /// 批量删除数据
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, ErrorMsg> {
        let result = self
            .user_role_rel_dao
            .batch_delete(ids)
            .await
            .map_err(|err| {
                error!("批量删除用户角色关系失败, err: {:#?}", err);
                Error::DbBatchDeleteError.into_err_with_msg("批量删除用户角色关系失败")
            })?;

        Ok(result)
    }
}
