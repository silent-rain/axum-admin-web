//! API操作日志

use log::error;
use nject::injectable;
use sea_orm::Set;

use entity::log::log_api_operation;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::api_operation::ApiOperationDao,
    dto::api_operation::{
        CreateApiOperationReq, DeleteApiOperationReq, GetApiOperationReq, GetApiOperationsReq,
    },
};

/// 服务层
#[injectable]
pub struct ApiOperationService {
    system_dao: ApiOperationDao,
}

impl ApiOperationService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetApiOperationsReq,
    ) -> Result<(Vec<log_api_operation::Model>, u64), ErrorMsg> {
        let (results, total) = self.system_dao.list(req).await.map_err(|err| {
            error!("查询操作日志列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询操作日志列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(
        &self,
        req: GetApiOperationReq,
    ) -> Result<log_api_operation::Model, ErrorMsg> {
        let result = self
            .system_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询操作日志失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询操作日志失败")
            })?
            .ok_or_else(|| {
                error!("操作日志不存在");
                Error::DbQueryEmptyError.into_err_with_msg("操作日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        req: CreateApiOperationReq,
    ) -> Result<log_api_operation::Model, ErrorMsg> {
        let data = log_api_operation::ActiveModel {
            user_id: Set(req.user_id),
            username: Set(req.username),
            request_id: Set(req.request_id),
            status_code: Set(req.status_code),
            method: Set(req.method),
            path: Set(req.path),
            content_type: Set(req.content_type),
            query: Set(req.query),
            body: Set(req.body),
            remote_addr: Set(req.remote_addr),
            user_agent: Set(req.user_agent),
            cost: Set(req.cost),
            http_type: Set(req.http_type.into()),
            desc: Set(req.desc),
            ..Default::default()
        };
        let result = self.system_dao.create(data).await.map_err(|err| {
            error!("添加操作日志失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("添加操作日志失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteApiOperationReq) -> Result<u64, ErrorMsg> {
        let result = self.system_dao.delete(req.id).await.map_err(|err| {
            error!("删除操作日志失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("删除操作日志失败")
        })?;

        Ok(result)
    }
}
