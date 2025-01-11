//! 任务调度作业表
//! Entity: [`entity::schedule::ScheduleJob`]

use sea_orm::{ConnectionTrait, DatabaseBackend, DeriveMigrationName};
use sea_orm_migration::{async_trait, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match manager.get_database_backend() {
            DatabaseBackend::MySql => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_schedule_job` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
                        `name` VARCHAR(200) UNIQUE NOT NULL COMMENT '任务名称',
                        `source` TINYINT(1) NOT NULL COMMENT '任务来源(0:用户定义,1:系统内部)',
                        `job_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务类型(0:定时任务,1:即时任务)',
                        `sys_code` VARCHAR(200) NOT NULL COMMENT '系统任务编码',
                        `expression` VARCHAR(100) DEFAULT '' COMMENT 'cron表达式',
                        `interval` INT(11) DEFAULT 0 COMMENT '间隔时间,秒',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '任务状态(0:下线,1:上线)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`) USING BTREE
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '任务调度作业表';

                    CREATE INDEX idx_sys_code ON t_schedule_job (`sys_code`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_schedule_job" (
                        "id" SERIAL PRIMARY KEY,
                        "name" VARCHAR(200) UNIQUE NOT NULL,
                        "source" BOOLEAN NOT NULL,
                        "job_type" BOOLEAN NOT NULL DEFAULT FALSE,
                        "sys_code" VARCHAR(200) NOT NULL,
                        "expression" VARCHAR(100) DEFAULT '',
                        "interval" INTEGER DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" char NOT NULL DEFAULT 1,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_schedule_job_sys_code ON t_schedule_job ("sys_code");

                    COMMENT ON TABLE t_schedule_job IS '任务调度作业表';
                    COMMENT ON COLUMN t_schedule_job.id IS '自增ID';
                    COMMENT ON COLUMN t_schedule_job.name IS '任务名称';
                    COMMENT ON COLUMN t_schedule_job.source IS '任务来源(0:用户定义,1:系统内部)';
                    COMMENT ON COLUMN t_schedule_job.job_type IS '任务类型(0:定时任务,1:即时任务)';
                    COMMENT ON COLUMN t_schedule_job.sys_code IS '系统任务编码';
                    COMMENT ON COLUMN t_schedule_job.expression IS 'cron表达式';
                    COMMENT ON COLUMN t_schedule_job.interval IS '间隔时间,秒';
                    COMMENT ON COLUMN t_schedule_job.desc IS '描述信息';
                    COMMENT ON COLUMN t_schedule_job.status IS '任务状态(0:下线,1:上线)';
                    COMMENT ON COLUMN t_schedule_job.created_at IS '创建时间';
                    COMMENT ON COLUMN t_schedule_job.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_schedule_job` ( -- 任务调度作业表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
                        `name` TEXT UNIQUE NOT NULL, -- 任务名称
                        `source` BOOLEAN NOT NULL, -- 任务来源(0:用户定义,1:系统内部)
                        `job_type` BOOLEAN NOT NULL DEFAULT 0, -- 任务类型(0:定时任务,1:即时任务)
                        `sys_code` TEXT NOT NULL, -- 系统任务编码
                        `expression` TEXT DEFAULT '', -- cron表达式
                        `interval` INTEGER DEFAULT 0, -- 间隔时间,秒
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `status` TINYINT(1) NOT NULL DEFAULT 1, -- 任务状态(0:下线,1:上线)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );
                    
                    CREATE INDEX idx_t_schedule_job_sys_code ON t_schedule_job (`sys_code`);
                    ",
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE `t_schedule_job`")
            .await?;

        Ok(())
    }
}
