//! 系统日志

use log::error;
use nject::injectable;

use entity::log::log_system;
use err_code::{Error, ErrorMsg};
use utils::json::struct_to_struct;

use crate::{
    dao::system_log::SystemLogDao,
    dto::system_log::{CreateSystemLogReq, DeleteSystemLogReq, GetSystemLogReq, GetSystemLogsReq},
};

/// 服务层
#[injectable]
pub struct SystemLogService {
    system_dao: SystemLogDao,
}

impl SystemLogService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetSystemLogsReq,
    ) -> Result<(Vec<log_system::Model>, u64), ErrorMsg> {
        let (results, total) = self.system_dao.list(req).await.map_err(|err| {
            error!("查询系统日志列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询系统日志列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetSystemLogReq) -> Result<log_system::Model, ErrorMsg> {
        let result = self
            .system_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询系统日志失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询系统日志失败")
            })?
            .ok_or_else(|| {
                error!("系统日志不存在");
                Error::DbQueryEmptyError.into_err_with_msg("系统日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateSystemLogReq) -> Result<log_system::Model, ErrorMsg> {
        let data: log_system::Model = struct_to_struct(&req).map_err(|err| {
            error!("JSON转换错误失败, err: {:#?}", err);
            Error::JsonConvert(err.to_string()).into_err_with_msg("JSON转换错误失败")
        })?;

        let result = self.system_dao.create(data).await.map_err(|err| {
            error!("添加系统日志失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("添加系统日志失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteSystemLogReq) -> Result<u64, ErrorMsg> {
        let result = self.system_dao.delete(req.id).await.map_err(|err| {
            error!("删除系统日志失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("删除系统日志失败")
        })?;

        Ok(result)
    }
}
