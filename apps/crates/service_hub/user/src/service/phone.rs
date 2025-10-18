//! 用户手机号管理

use log::error;
use nject::injectable;
use sea_orm::Set;

use entity::user::phone;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::phone::PhoneDao,
    dto::phone::{CreatePhoneReq, DeletePhoneReq, GetPhoneReq, GetPhonesReq, UpdatePhoneReq},
};

/// 服务层
#[injectable]
pub struct PhoneService {
    phone_dao: PhoneDao,
}

impl PhoneService {
    /// 获取列表数据
    pub async fn list(&self, req: GetPhonesReq) -> Result<(Vec<phone::Model>, u64), ErrorMsg> {
        let (results, total) = self.phone_dao.list(req).await.map_err(|err| {
            error!("查询用户手机号列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询用户手机号列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetPhoneReq) -> Result<phone::Model, ErrorMsg> {
        let result = self
            .phone_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询用户手机号信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户手机号信息失败")
            })?
            .ok_or_else(|| {
                error!("用户手机号不存在");
                Error::DbQueryEmptyError.into_err_with_msg("用户手机号不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreatePhoneReq) -> Result<phone::Model, ErrorMsg> {
        // 检查用户手机号是否已存在
        self.check_phone_exist(req.phone.clone(), None).await?;

        let model = phone::ActiveModel {
            user_id: Set(req.user_id),
            phone: Set(req.phone),
            desc: Set(req.desc),
            ..Default::default()
        };
        let result = self.phone_dao.create(model).await.map_err(|err| {
            error!("添加用户手机号信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加用户手机号信息失败")
        })?;

        Ok(result)
    }

    /// 更新用户手机号
    pub async fn update(&self, req: UpdatePhoneReq) -> Result<u64, ErrorMsg> {
        // 检查用户手机号是否已存在且不属于当前ID
        self.check_phone_exist(req.phone.clone(), Some(req.id))
            .await?;

        let model = phone::ActiveModel {
            id: Set(req.id),
            phone: Set(req.phone),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self.phone_dao.update(model).await.map_err(|err| {
            error!("更新用户手机号失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新用户手机号失败")
        })?;

        Ok(result)
    }

    /// 检查手机号是否存在
    async fn check_phone_exist(
        &self,
        phone: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self.phone_dao.info_by_phone(phone).await.map_err(|err| {
            error!("查询用户手机号信息失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询用户手机号信息失败")
        })?;

        // 存在
        if let Some(model) = result
            && (current_id.is_none() || Some(model.id) != current_id)
        {
            error!("户手机号已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("户手机号已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeletePhoneReq) -> Result<u64, ErrorMsg> {
        let result = self.phone_dao.delete(req.id).await.map_err(|err| {
            error!("删除用户手机号失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除用户手机号失败")
        })?;

        Ok(result)
    }
}
