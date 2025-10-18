//! 用户地理位置管理

use log::error;
use nject::injectable;
use sea_orm::Set;

use entity::user::location;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::location::LocationDao,
    dto::location::{
        CreateLocationReq, DeleteLocationReq, GetLocationReq, GetLocationsReq, UpdateLocationReq,
    },
};

/// 服务层
#[injectable]
pub struct LocationService {
    location_dao: LocationDao,
}

impl LocationService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetLocationsReq,
    ) -> Result<(Vec<location::Model>, u64), ErrorMsg> {
        let (results, total) = self.location_dao.list(req).await.map_err(|err| {
            error!("查询用户地理位置列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询用户地理位置列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetLocationReq) -> Result<location::Model, ErrorMsg> {
        let result = self
            .location_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询用户地理位置信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户地理位置信息失败")
            })?
            .ok_or_else(|| {
                error!("用户地理位置不存在");
                Error::DbQueryEmptyError.into_err_with_msg("用户地理位置不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateLocationReq) -> Result<location::Model, ErrorMsg> {
        // 查询用户地理位置是否已存在
        let location = self
            .location_dao
            .info_user_id(req.user_id)
            .await
            .map_err(|err| {
                error!("查询用户地理位置信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户地理位置信息失败")
            })?;
        if location.is_some() {
            error!("用户地理位置已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("用户地理位置已存在"));
        }

        let model = location::ActiveModel {
            user_id: Set(req.user_id),
            province: Set(req.province),
            city: Set(req.city),
            district: Set(req.district),
            address: Set(req.address),
            postal_code: Set(req.postal_code),
            longitude: Set(req.longitude),
            latitude: Set(req.latitude),
            desc: Set(req.desc),
            ..Default::default()
        };
        let result = self.location_dao.create(model).await.map_err(|err| {
            error!("添加用户地理位置信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加用户地理位置信息失败")
        })?;

        Ok(result)
    }

    /// 更新用户地理位置
    pub async fn update(&self, req: UpdateLocationReq) -> Result<u64, ErrorMsg> {
        let model = location::ActiveModel {
            id: Set(req.id),
            province: Set(req.province),
            city: Set(req.city),
            district: Set(req.district),
            address: Set(req.address),
            postal_code: Set(req.postal_code),
            longitude: Set(req.longitude),
            latitude: Set(req.latitude),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self.location_dao.update(model).await.map_err(|err| {
            error!("更新用户地理位置失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新用户地理位置失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteLocationReq) -> Result<u64, ErrorMsg> {
        let result = self.location_dao.delete(req.id).await.map_err(|err| {
            error!("删除用户地理位置信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除用户地理位置信息失败")
        })?;

        Ok(result)
    }
}
