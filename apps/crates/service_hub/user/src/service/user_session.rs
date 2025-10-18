//! 用户session管理

use log::error;
use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};

use entity::user::user_session;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::user_session::UserSessionDao,
    dto::user_session::{
        CreateUserSessionReq, DeleteUserSessionReq, GetUserSessionReq, GetUserSessionsReq,
        UpdateUserSessionReq, UpdateUserSessionStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct UserSessionService {
    user_session_dao: UserSessionDao,
}

impl UserSessionService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetUserSessionsReq,
    ) -> Result<(Vec<user_session::Model>, u64), ErrorMsg> {
        let (results, total) = self.user_session_dao.list(req).await.map_err(|err| {
            error!("查询用户session列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询用户session列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetUserSessionReq) -> Result<user_session::Model, ErrorMsg> {
        let result = self
            .user_session_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询用户session信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户session信息失败")
            })?
            .ok_or_else(|| {
                error!("用户session不存在");
                Error::DbQueryEmptyError.into_err_with_msg("用户session不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateUserSessionReq) -> Result<user_session::Model, ErrorMsg> {
        let model = user_session::ActiveModel {
            session_id: Set(req.session_id),
            expiry_date: Set(req.expiry_date),
            data: Set(req.data),
            ..Default::default()
        };
        let result = self.user_session_dao.create(model).await.map_err(|err| {
            error!("添加用户session信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加用户session信息失败")
        })?;

        Ok(result)
    }

    /// 更新用户session
    pub async fn update(&self, req: UpdateUserSessionReq) -> Result<u64, ErrorMsg> {
        let model = user_session::ActiveModel {
            id: Set(req.id),
            expiry_date: Set(req.expiry_date),
            data: Set(req.data),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self.user_session_dao.update(model).await.map_err(|err| {
            error!("更新用户session失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新用户session失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateUserSessionStatusReq) -> Result<(), ErrorMsg> {
        self.user_session_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新用户session状态失败, {err}");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新用户session状态失败, 请登陆");
                }
                error!("更新用户session状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("操作失败, 请重新登陆")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteUserSessionReq) -> Result<u64, ErrorMsg> {
        let result = self.user_session_dao.delete(req.id).await.map_err(|err| {
            error!("删除用户session失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除用户session失败")
        })?;

        Ok(result)
    }
}
