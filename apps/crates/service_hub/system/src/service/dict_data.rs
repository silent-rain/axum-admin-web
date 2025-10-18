//! 字典数据管理

use log::error;
use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};

use entity::system::dict_data;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::dict_data::DictDataDao,
    dto::dict_data::{
        CreateDictDataReq, DeleteDictDataReq, GetDictDataReq, GetDictDatasReq, UpdateDictDataReq,
        UpdateDictDataStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct DictDataService {
    dict_data_dao: DictDataDao,
}

impl DictDataService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetDictDatasReq,
    ) -> Result<(Vec<dict_data::Model>, u64), ErrorMsg> {
        let (results, total) = self.dict_data_dao.list(req).await.map_err(|err| {
            error!("查询字典数据列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询字典数据列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetDictDataReq) -> Result<dict_data::Model, ErrorMsg> {
        let result = self
            .dict_data_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询字典数据信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询字典数据信息失败")
            })?
            .ok_or_else(|| {
                error!("字典数据不存在");
                Error::DbQueryEmptyError.into_err_with_msg("字典数据不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateDictDataReq) -> Result<dict_data::Model, ErrorMsg> {
        // 查询字典数据是否已存在
        let dict_data = self
            .dict_data_dao
            .info_by_lable(req.dim_id, req.lable.clone())
            .await
            .map_err(|err| {
                error!("查询字典标签失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询字典标签失败")
            })?;
        if dict_data.is_some() {
            error!("字典标签已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("字典标签已存在"));
        }

        let model = dict_data::ActiveModel {
            dim_id: Set(req.dim_id),
            lable: Set(req.lable),
            value: Set(req.value),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(true),
            ..Default::default()
        };
        let result = self.dict_data_dao.create(model).await.map_err(|err| {
            error!("添加字典数据信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加字典数据信息失败")
        })?;

        Ok(result)
    }

    /// 更新字典数据
    pub async fn update(&self, req: UpdateDictDataReq) -> Result<u64, ErrorMsg> {
        let model = dict_data::ActiveModel {
            id: Set(req.id),
            lable: Set(req.lable),
            value: Set(req.value),
            sort: Set(req.sort),
            desc: Set(req.desc),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self.dict_data_dao.update(model).await.map_err(|err| {
            error!("更新字典数据失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新字典数据失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateDictDataStatusReq) -> Result<(), ErrorMsg> {
        self.dict_data_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新字典数据状态失败, 该字典数据不存在");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新字典数据状态失败, 该字典数据不存在");
                }
                error!("更新字典数据状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新字典数据状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteDictDataReq) -> Result<u64, ErrorMsg> {
        let result = self.dict_data_dao.delete(req.id).await.map_err(|err| {
            error!("删除字典数据信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除字典数据信息失败")
        })?;

        Ok(result)
    }
}
