//! 会员等级管理

use log::error;
use nject::injectable;
use sea_orm::Set;

use entity::user::member_level;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::member_level::MemberLevelDao,
    dto::member_level::{
        CreateMemberLevelReq, DeleteMemberLevelReq, GetMemberLevelReq, GetMemberLevelsReq,
        UpdateMemberLevelReq, UpdateMemberLevelStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct MemberLevelService {
    member_level_dao: MemberLevelDao,
}

impl MemberLevelService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetMemberLevelsReq,
    ) -> Result<(Vec<member_level::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.member_level_dao.all().await.map_err(|err| {
                error!("查询所有会员等级失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询所有会员等级失败")
            });
        }

        let (results, total) = self.member_level_dao.list(req).await.map_err(|err| {
            error!("查询会员等级列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询会员等级列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetMemberLevelReq) -> Result<member_level::Model, ErrorMsg> {
        let result = self
            .member_level_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询会员等级信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询会员等级信息失败")
            })?
            .ok_or_else(|| {
                error!("会员等级不存在");
                Error::DbQueryEmptyError.into_err_with_msg("会员等级不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateMemberLevelReq) -> Result<member_level::Model, ErrorMsg> {
        // 查询会员等级名称是否已存在
        self.check_name(req.name.clone(), None).await?;
        // 检查会员等级是否已存在且不属于当前ID
        self.check_level(req.level, None).await?;

        let model = member_level::ActiveModel {
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            level: Set(req.level),
            status: Set(req.status),
            ..Default::default()
        };
        let member_level =
            self.member_level_dao
                .create(model)
                .await
                .map_err(|err: sea_orm::prelude::DbErr| {
                    error!("添加会员等级信息失败, err: {:#?}", err);
                    Error::DbAddError.into_err_with_msg("添加会员等级信息失败")
                })?;

        Ok(member_level)
    }

    /// 更新数据
    pub async fn update(&self, req: UpdateMemberLevelReq) -> Result<u64, ErrorMsg> {
        // 查询会员等级名称是否已存在且不属于当前ID
        self.check_name(req.name.clone(), Some(req.id)).await?;
        // 检查会员等级是否已存在且不属于当前ID
        self.check_level(req.level, Some(req.id)).await?;

        let model = member_level::ActiveModel {
            id: Set(req.id),
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            level: Set(req.level),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self.member_level_dao.update(model).await.map_err(|err| {
            error!("更新会员等级失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新会员等级失败")
        })?;

        Ok(result)
    }

    /// 查询会员等级名称是否已存在
    async fn check_name(&self, name: String, current_id: Option<i32>) -> Result<(), ErrorMsg> {
        let result = self
            .member_level_dao
            .info_by_name(name)
            .await
            .map_err(|err| {
                error!("查询会员等级信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询会员等级信息失败")
            })?;

        // 存在
        if let Some(model) = result
            && (current_id.is_none() || Some(model.id) != current_id)
        {
            error!("会员等级名称已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("会员等级名称已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 检查会员等级是否存在
    async fn check_level(&self, level: u16, current_id: Option<i32>) -> Result<(), ErrorMsg> {
        let result = self
            .member_level_dao
            .info_by_level(level)
            .await
            .map_err(|err| {
                error!("查询会员等级失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询会员等级失败")
            })?;

        // 存在
        if let Some(model) = result
            && (current_id.is_none() || Some(model.id) != current_id)
        {
            error!("会员等级已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("会员等级已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateMemberLevelStatusReq) -> Result<(), ErrorMsg> {
        self.member_level_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                error!("更新会员等级状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新会员等级状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteMemberLevelReq) -> Result<u64, ErrorMsg> {
        let result = self.member_level_dao.delete(req.id).await.map_err(|err| {
            error!("删除会员等级信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除会员等级信息失败")
        })?;

        Ok(result)
    }
}
