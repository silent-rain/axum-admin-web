//! 任务调度状态日志管理
use crate::{
    dao::schedule_status_log::ScheduleStatusLogDao,
    dto::schedule_status_log::{
        AddScheduleStatusLogReq, GetScheduleStatusLogListLogReq, UpdateScheduleStatusLogReq,
    },
};

use code::{Error, ErrorMsg};
use entity::schedule::schedule_status_log;

use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};
use tracing::error;

/// 服务层
#[injectable]
pub struct ScheduleStatusLogService {
    schedule_status_log_dao: ScheduleStatusLogDao,
}

impl ScheduleStatusLogService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetScheduleStatusLogListLogReq,
    ) -> Result<(Vec<schedule_status_log::Model>, u64), ErrorMsg> {
        let (results, total) = self
            .schedule_status_log_dao
            .list(req)
            .await
            .map_err(|err| {
                error!("查询任务调度状态日志列表失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询任务调度状态日志列表失败")
            })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, id: i32) -> Result<schedule_status_log::Model, ErrorMsg> {
        let result = self
            .schedule_status_log_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询任务调度状态日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询任务调度状态日志失败")
            })?
            .ok_or_else(|| {
                error!("任务调度状态日志不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("任务调度状态日志不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn add(
        &self,
        req: AddScheduleStatusLogReq,
    ) -> Result<schedule_status_log::Model, ErrorMsg> {
        let data = schedule_status_log::ActiveModel {
            job_id: Set(req.job_id),
            uuid: Set(req.uuid),
            ..Default::default()
        };
        let result = self
            .schedule_status_log_dao
            .add(data)
            .await
            .map_err(|err| {
                error!("添加任务调度状态日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("添加任务调度状态日志失败")
            })?;

        Ok(result)
    }

    /// 更新数据
    pub async fn update(&self, id: i32, req: UpdateScheduleStatusLogReq) -> Result<u64, ErrorMsg> {
        let model = schedule_status_log::ActiveModel {
            id: Set(id),
            job_id: Set(req.job_id),
            error: Set(req.error),
            cost: Set(req.cost),
            status: Set(req.status as i8),
            ..Default::default()
        };

        let result = self
            .schedule_status_log_dao
            .update(model)
            .await
            .map_err(|err| {
                error!("更新任务调度状态日志失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新任务调度状态日志失败")
            })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), ErrorMsg> {
        self.schedule_status_log_dao
            .status(id, status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新任务调度状态日志失败, 该任务调度状态日志不存在");
                    return Error::DbUpdateError
                        .into_msg()
                        .with_msg("更新任务调度状态日志失败, 该任务调度状态日志不存在");
                }
                error!("更新任务调度状态日志失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新任务调度状态日志失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, id: i32) -> Result<u64, ErrorMsg> {
        let result = self
            .schedule_status_log_dao
            .delete(id)
            .await
            .map_err(|err| {
                error!("删除任务调度状态日志失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("删除任务调度状态日志失败")
            })?;

        Ok(result)
    }
}
