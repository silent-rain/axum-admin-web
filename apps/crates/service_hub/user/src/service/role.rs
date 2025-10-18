//! 角色管理

use log::error;
use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};

use entity::user::role;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::role::RoleDao,
    dto::role::{
        CreateRoleReq, DeleteRoleReq, GetRoleReq, GetRolesReq, UpdateRoleReq, UpdateRoleStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct RoleService {
    role_dao: RoleDao,
}

impl RoleService {
    /// 获取列表数据
    pub async fn list(&self, req: GetRolesReq) -> Result<(Vec<role::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.role_dao.all().await.map_err(|err| {
                error!("查询角色列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询角色列表失败")
            });
        }

        let (results, total) = self.role_dao.list(req).await.map_err(|err| {
            error!("查询角色列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询角色列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetRoleReq) -> Result<role::Model, ErrorMsg> {
        let result = self
            .role_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询角色信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询角色信息失败")
            })?
            .ok_or_else(|| {
                error!("角色不存在");
                Error::DbQueryEmptyError.into_err_with_msg("角色不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateRoleReq) -> Result<role::Model, ErrorMsg> {
        // 检查角色名称是否已存在
        self.check_name_exist(req.name.clone(), None).await?;

        let model = role::ActiveModel {
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(true),
            ..Default::default()
        };
        let result = self.role_dao.create(model).await.map_err(|err| {
            error!("添加角色信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加角色信息失败")
        })?;

        Ok(result)
    }

    /// 更新角色
    pub async fn update(&self, req: UpdateRoleReq) -> Result<u64, ErrorMsg> {
        // 检查角色名称是否已存在且不属于当前ID
        self.check_name_exist(req.name.clone(), Some(req.id))
            .await?;

        let model = role::ActiveModel {
            id: Set(req.id),
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self.role_dao.update(model).await.map_err(|err| {
            error!("更新角色失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新角色失败")
        })?;

        Ok(result)
    }

    /// 检查角色名称是否存在
    async fn check_name_exist(
        &self,
        name: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self.role_dao.info_by_name(name).await.map_err(|err| {
            error!("查询角色信息失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询角色信息失败")
        })?;

        // 存在
        if let Some(model) = result
            && (current_id.is_none() || Some(model.id) != current_id)
        {
            error!("角色名称已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("角色名称已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateRoleStatusReq) -> Result<(), ErrorMsg> {
        self.role_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新角色状态失败, 该角色不存在");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新角色状态失败, 该角色不存在");
                }
                error!("更新角色状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新角色状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteRoleReq) -> Result<u64, ErrorMsg> {
        let result = self.role_dao.delete(req.id).await.map_err(|err| {
            error!("删除角色信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除角色信息失败")
        })?;

        Ok(result)
    }
}
