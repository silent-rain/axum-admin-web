//! # 任务管理与注册
//! - 初始化所有系统任务
//!     - 获取系统任务
//!     - 注册系统任务
//!     - 添加任务运行状态日志
//! - 初始化所有的脚本任务
//!     - 获取用户任务
//!     - 注册用户任务
//!     - 添加任务运行状态日志
use crate::{dao::Dao, error::Error, Job, JobScheduler};

use database::PoolTrait;
use entity::schedule::schedule_job;

use async_trait::async_trait;
use tracing::{error, info};

/// 系统定时任务 Trait
#[async_trait]
pub trait SysTaskTrait<DB>
where
    DB: PoolTrait + Clone + Send + Sync + 'static,
    Self: Send + Sync + 'static,
{
    /// 系统定时任务编码
    fn sys_code(&self) -> String;
    /// 即时任务
    fn task_interval(&self, _sys_id: i32, _interval: i32) -> Result<Job<DB>, Error> {
        Err(Error::NotInitJob)
    }
    /// 定时任务
    fn task_cron(&self, _sys_id: i32, _expression: String) -> Result<Job<DB>, Error> {
        Err(Error::NotInitJob)
    }
}

/// 系统定时任务注册
pub struct SysTaskRegister<DB>
where
    DB: PoolTrait + Clone + Send + Sync + 'static,
{
    dao: Dao<DB>,
    tasks: Vec<Box<dyn SysTaskTrait<DB>>>,
}

impl<DB> SysTaskRegister<DB>
where
    DB: PoolTrait + Clone + Send + Sync + 'static,
{
    pub fn new(db: DB) -> Self {
        SysTaskRegister {
            dao: Dao::new(db),
            tasks: Vec::new(),
        }
    }

    /// 注册任务
    pub async fn register(&mut self) -> Result<(), Error> {
        let sys_job_list = self.sys_job_list().await?;

        for task in self.tasks.iter() {
            let job_model = sys_job_list
                .iter()
                .find(|v| v.sys_code == Some(task.sys_code()));

            // 更新数据库中的任务UUID
            let job_model = match job_model {
                Some(v) => v,
                None => continue,
            };
            // 判读配置是否为系统任务
            if job_model.source != schedule_job::enums::Source::System as i8 {
                continue;
            }

            let sys_job = if job_model.job_type == schedule_job::enums::JobType::Interval as i8 {
                let interval = job_model.interval.ok_or(Error::NotIntervalError)?;
                task.task_interval(job_model.id, interval)?
            } else if job_model.job_type == schedule_job::enums::JobType::Timer as i8 {
                let expression = job_model
                    .expression
                    .clone()
                    .ok_or(Error::NotExpressionError)?;
                task.task_cron(job_model.id, expression)?
            } else {
                continue;
            };

            // 将job添加到调度任务中
            self.add_sched_job(sys_job, job_model.clone()).await?;
        }

        Ok(())
    }

    /// 添加任务
    pub fn add_task(&mut self, task: Box<dyn SysTaskTrait<DB>>) {
        self.tasks.push(task);
    }

    /// 将job添加到调度任务中
    async fn add_sched_job(
        &self,
        sys_job: Job<DB>,
        job_model: schedule_job::Model,
    ) -> Result<(), Error> {
        let uuid = sys_job.guid().to_string();
        info!(
            "register sys task id:{} name: {} sys_code: {:?} status: {}uuid: {:?}",
            job_model.id, job_model.name, job_model.sys_code, job_model.status, uuid,
        );

        // 创建任务
        let sched = JobScheduler::new().await?;
        // 将任务添加到任务队列中
        sched.add_job(sys_job.clone()).await?;

        Ok(())
    }

    /// 获取所有的系统定时任务
    async fn sys_job_list(&self) -> Result<Vec<schedule_job::Model>, Error> {
        let job_list = self
            .dao
            .schedule_job_dao
            .list()
            .await
            .map_err(|err| Error::ScheduleJobListError(err.to_string()))?
            .into_iter()
            .filter(|v| v.source == schedule_job::enums::Source::System as i8)
            .collect::<Vec<schedule_job::Model>>();
        Ok(job_list)
    }
}

/// 用户定时任务注册
pub struct UserTaskRegister<DB>
where
    DB: PoolTrait + Clone + Send + Sync + 'static,
{
    db: DB,
    dao: Dao<DB>,
}

impl<DB> UserTaskRegister<DB>
where
    DB: PoolTrait + Clone + Send + Sync + 'static,
{
    pub fn new(db: DB) -> Self {
        UserTaskRegister {
            db: db.clone(),
            dao: Dao::new(db),
        }
    }

    /// 注册任务
    pub async fn register(&mut self) -> Result<(), Error> {
        let job_list = self.user_job_list().await?;

        for job_model in job_list.iter() {
            let user_job = if job_model.job_type == schedule_job::enums::JobType::Interval as i8 {
                self.init_interval_task(job_model)?
            } else {
                self.init_cron_task(job_model)?
            };
            let uuid = user_job.guid().to_string();
            info!(
                "register user task id:{} name: {} sys_code: {:?} status: {:?} uuid: {:?}",
                job_model.id, job_model.name, job_model.sys_code, job_model.status, uuid
            );

            // 将任务添加到任务队列中
            JobScheduler::new().await?.add_job(user_job.clone()).await?;
            let sched = match JobScheduler::new().await {
                Ok(v) => v,
                Err(err) => {
                    error!(
                        "id:{} task name: {} sys_code: {:?}, err: {}",
                        job_model.id,
                        job_model.name,
                        job_model.sys_code,
                        err.to_string()
                    );
                    continue;
                }
            };
            match sched.add_job(user_job.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    error!(
                        "id:{} task name: {} sys_code: {:?}, err: {}",
                        job_model.id,
                        job_model.name,
                        job_model.sys_code,
                        err.to_string()
                    );
                    continue;
                }
            };
        }

        Ok(())
    }

    /// 初始化定时任务
    fn init_cron_task(&self, model: &schedule_job::Model) -> Result<Job<DB>, Error> {
        let expression = model.expression.clone().ok_or(Error::NotExpressionError)?;
        let job = Job::new(1, self.db.clone())?.with_cron_job(&expression, |uuid, _jobs| {
            Box::pin(async move {
                // TODO 执行脚本
                println!("I run async every 5 seconds uuid: {uuid} job11");
                Ok(())
            })
        })?;
        Ok(job)
    }

    /// 初始化即时任务
    fn init_interval_task(&self, model: &schedule_job::Model) -> Result<Job<DB>, Error> {
        let interval = model.interval.ok_or(Error::NotExpressionError)? as u64;
        let job = Job::new(1, self.db.clone())?.with_interval_job(interval, |uuid, _jobs| {
            Box::pin(async move {
                // TODO 执行脚本
                println!("I run async every 5 seconds uuid: {uuid} job11");
                Ok(())
            })
        })?;

        Ok(job)
    }

    /// 获取所有的用户定时任务
    async fn user_job_list(&self) -> Result<Vec<schedule_job::Model>, Error> {
        let job_list = self
            .dao
            .schedule_job_dao
            .list()
            .await
            .map_err(|err| Error::ScheduleJobListError(err.to_string()))?
            .into_iter()
            .filter(|v| v.source == schedule_job::enums::Source::User as i8)
            .collect::<Vec<schedule_job::Model>>();

        Ok(job_list)
    }
}
