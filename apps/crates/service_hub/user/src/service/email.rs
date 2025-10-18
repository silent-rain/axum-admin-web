//! 用户邮箱管理

use log::error;
use nject::injectable;
use sea_orm::Set;

use entity::user::email;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::email::EmailDao,
    dto::email::{CreateEmailReq, DeleteEmailReq, GetEmailReq, GetEmailsReq, UpdateEmailReq},
};

/// 服务层
#[injectable]
pub struct EmailService {
    email_dao: EmailDao,
}

impl EmailService {
    /// 获取列表数据
    pub async fn list(&self, req: GetEmailsReq) -> Result<(Vec<email::Model>, u64), ErrorMsg> {
        let (results, total) = self.email_dao.list(req).await.map_err(|err| {
            error!("查询邮箱列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询邮箱列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetEmailReq) -> Result<email::Model, ErrorMsg> {
        let result = self
            .email_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询邮箱信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询邮箱信息失败")
            })?
            .ok_or_else(|| {
                error!("邮箱不存在");
                Error::DbQueryEmptyError.into_err_with_msg("邮箱不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateEmailReq) -> Result<email::Model, ErrorMsg> {
        // 查询邮箱是否已存在
        let email = self
            .email_dao
            .info_by_email(req.email.clone())
            .await
            .map_err(|err| {
                error!("查询邮箱信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询邮箱信息失败")
            })?;
        if email.is_some() {
            error!("邮箱已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("邮箱已存在"));
        }

        // 检查邮箱名称是否已存在
        self.check_email_exist(req.email.clone(), None).await?;

        let model = email::ActiveModel {
            user_id: Set(req.user_id),
            email: Set(req.email),
            desc: Set(req.desc),
            ..Default::default()
        };
        let result = self.email_dao.create(model).await.map_err(|err| {
            error!("添加邮箱信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加邮箱信息失败")
        })?;

        Ok(result)
    }

    /// 更新邮箱
    pub async fn update(&self, req: UpdateEmailReq) -> Result<u64, ErrorMsg> {
        // 检查邮箱是否已存在且不属于当前ID
        self.check_email_exist(req.email.clone(), Some(req.id))
            .await?;

        let model = email::ActiveModel {
            id: Set(req.id),
            email: Set(req.email),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self.email_dao.update(model).await.map_err(|err| {
            error!("更新邮箱失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新邮箱失败")
        })?;

        Ok(result)
    }

    /// 检查邮箱是否存在
    async fn check_email_exist(
        &self,
        email: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self.email_dao.info_by_email(email).await.map_err(|err| {
            error!("查询邮箱信息失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询邮箱信息失败")
        })?;

        // 存在
        if let Some(model) = result
            && (current_id.is_none() || Some(model.id) != current_id)
        {
            error!("邮箱已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("邮箱已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteEmailReq) -> Result<u64, ErrorMsg> {
        let result = self.email_dao.delete(req.id).await.map_err(|err| {
            error!("删除邮箱失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除邮箱失败")
        })?;

        Ok(result)
    }
}
