//! 岗位管理
use crate::{
    dao::position::PositionDao,
    dto::position::{
        CreatePositionReq, DeletePositionReq, GetPositionReq, GetPositionsReq, UpdatePositionReq,
        UpdatePositionStatusReq,
    },
};

use code::{Error, ErrorMsg};
use entity::organization::position;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct PositionService {
    position_dao: PositionDao,
}

impl PositionService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetPositionsReq,
    ) -> Result<(Vec<position::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.position_dao.all().await.map_err(|err| {
                error!("查询所有岗位失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询所有岗位失败")
            });
        }

        let (results, total) = self.position_dao.list(req).await.map_err(|err| {
            error!("查询岗位列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询岗位列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetPositionReq) -> Result<position::Model, ErrorMsg> {
        let result = self
            .position_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询岗位信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询岗位信息失败")
            })?
            .ok_or_else(|| {
                error!("岗位不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("岗位不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreatePositionReq) -> Result<position::Model, ErrorMsg> {
        // 检查岗位名称是否已存在
        self.check_name_exist(req.name.clone(), None).await?;

        let model = position::ActiveModel {
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            department_id: Set(req.department_id),
            status: Set(req.status),
            ..Default::default()
        };
        let position =
            self.position_dao
                .create(model)
                .await
                .map_err(|err: sea_orm::prelude::DbErr| {
                    error!("添加岗位信息失败, err: {:#?}", err);
                    Error::DbAddError.into_msg().with_msg("添加岗位信息失败")
                })?;

        Ok(position)
    }

    /// 更新数据
    pub async fn update(&self, req: UpdatePositionReq) -> Result<u64, ErrorMsg> {
        // 检查岗位名称是否已存在且不属于当前ID
        self.check_name_exist(req.name.clone(), Some(req.id))
            .await?;

        let model = position::ActiveModel {
            id: Set(req.id),
            name: Set(req.name),
            sort: Set(req.sort),
            desc: Set(req.desc),
            department_id: Set(req.department_id),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self.position_dao.update(model).await.map_err(|err| {
            error!("更新岗位失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新岗位失败")
        })?;

        Ok(result)
    }

    /// 检查岗位名称是否存在
    async fn check_name_exist(
        &self,
        name: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self.position_dao.info_by_name(name).await.map_err(|err| {
            error!("查询岗位信息失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询岗位信息失败")
        })?;

        // 存在
        if let Some(model) = result {
            if current_id.is_none() || Some(model.id) != current_id {
                error!("岗位名称已存在");
                return Err(Error::DbDataExistError
                    .into_msg()
                    .with_msg("岗位名称已存在"));
            }
        }

        // 不存在
        Ok(())
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdatePositionStatusReq) -> Result<(), ErrorMsg> {
        self.position_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                error!("更新岗位状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_msg().with_msg("更新岗位状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeletePositionReq) -> Result<u64, ErrorMsg> {
        let result = self.position_dao.delete(req.id).await.map_err(|err| {
            error!("删除岗位信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除岗位信息失败")
        })?;

        Ok(result)
    }
}
