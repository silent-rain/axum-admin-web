//! 登陆日志管理

use log::error;
use nject::injectable;
use sea_orm::Set;

use entity::user::user_login_log;
use err_code::{Error, ErrorMsg};
use utils::browser::parse_user_agent_async;

use crate::{
    dao::user_login_log::UserLoginLogDao,
    dto::user_login_log::{CreateUserLoginLogReq, GetUserLoginLogReq, GetUserLoginLogsReq},
};

/// 服务层
#[injectable]
pub struct UserLoginLogService {
    user_login_dao: UserLoginLogDao,
}

impl UserLoginLogService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetUserLoginLogsReq,
    ) -> Result<(Vec<user_login_log::Model>, u64), ErrorMsg> {
        let (mut results, total) = self.user_login_dao.list(req).await.map_err(|err| {
            error!("查询登陆日志列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询登陆日志列表失败")
        })?;

        // 重置 session_id 为空
        for item in results.iter_mut() {
            item.session_id = "".to_string();
        }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetUserLoginLogReq) -> Result<user_login_log::Model, ErrorMsg> {
        let mut result = self
            .user_login_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询登陆日志信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询登陆日志信息失败")
            })?
            .ok_or_else(|| {
                error!("登陆日志不存在");
                Error::DbQueryEmptyError.into_err_with_msg("登陆日志不存在")
            })?;

        result.session_id = "".to_string();
        Ok(result)
    }

    /// 根据SessionId获取详情信息
    pub async fn info_by_session_id(
        &self,
        session_id: String,
    ) -> Result<user_login_log::Model, ErrorMsg> {
        let mut result = self
            .user_login_dao
            .info_by_session_id(session_id)
            .await
            .map_err(|err| {
                error!("查询登陆日志信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询登陆日志信息失败")
            })?
            .ok_or_else(|| {
                error!("未查询到登陆信息，请重新登陆");
                Error::DbQueryEmptyError.into_err_with_msg("未查询到登陆信息，请重新登陆")
            })?;
        result.session_id = "".to_string();
        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        req: CreateUserLoginLogReq,
    ) -> Result<user_login_log::Model, ErrorMsg> {
        let (device, system, browser) = parse_user_agent_async(req.user_agent.clone())
            .await
            .map_err(|err| {
                error!("User-Agent解析错误, err: {:#?}", err);
                Error::UserAgentParserError(err).into_err_with_msg("User-Agent解析错误")
            })?;

        let model = user_login_log::ActiveModel {
            user_id: Set(req.user_id),
            username: Set(req.username),
            session_id: Set(req.session_id),
            remote_addr: Set(req.remote_addr),
            user_agent: Set(req.user_agent),
            device: Set(Some(device)),
            system: Set(Some(system)),
            browser: Set(Some(browser)),
            desc: Set(req.desc),
            login_status: Set(req.login_status as i8),
            ..Default::default()
        };
        let result = self.user_login_dao.create(model).await.map_err(|err| {
            error!("添加登陆日志信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加登陆日志信息失败")
        })?;

        Ok(result)
    }
}
