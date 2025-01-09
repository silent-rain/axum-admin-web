//! 任务调度状态日志表
//! Entity: [`entity::schedule::ScheduleStatusLog`]

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
                    `t_schedule_status_log` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '状态日志ID',
                        `job_id` INT(11) NOT NULL COMMENT '任务ID',
                        `uuid` VARCHAR(50) NOT NULL COMMENT '任务调度ID',
                        `error` TEXT COMMENT '失败信息',
                        `cost` INT(20) UNSIGNED NOT NULL COMMENT '耗时,毫秒',
                        `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务状态(0:开始,1:完成,2:停止,3:移除)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`) USING BTREE
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '任务调度状态日志表';

                    CREATE INDEX idx_job_id ON t_schedule_status_log (`job_id`);
                    CREATE INDEX idx_uuid ON t_schedule_status_log (`uuid`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_schedule_status_log" (
                        "id" SERIAL PRIMARY KEY,
                        "job_id" INTEGER NOT NULL,
                        "uuid" VARCHAR(50) NOT NULL,
                        "error" TEXT,
                        "cost" BIGINT NOT NULL,
                        "status" BOOLEAN NOT NULL DEFAULT FALSE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_schedule_status_log_job_id ON t_schedule_status_log ("job_id");
                    CREATE INDEX idx_t_schedule_status_log_uuid ON t_schedule_status_log ("uuid");

                    COMMENT ON TABLE t_schedule_status_log IS '任务调度状态日志表';
                    COMMENT ON COLUMN t_schedule_status_log.id IS '状态日志ID';
                    COMMENT ON COLUMN t_schedule_status_log.job_id IS '任务ID';
                    COMMENT ON COLUMN t_schedule_status_log.uuid IS '任务调度ID';
                    COMMENT ON COLUMN t_schedule_status_log.error IS '失败信息';
                    COMMENT ON COLUMN t_schedule_status_log.cost IS '耗时,毫秒';
                    COMMENT ON COLUMN t_schedule_status_log.status IS '任务状态(0:开始,1:完成,2:停止,3:移除)';
                    COMMENT ON COLUMN t_schedule_status_log.created_at IS '创建时间';
                    COMMENT ON COLUMN t_schedule_status_log.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_schedule_status_log` ( -- 任务调度状态日志表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 状态日志ID
                        `job_id` INTEGER NOT NULL, -- 任务ID
                        `uuid` TEXT NOT NULL, -- 任务调度ID
                        `error` TEXT, -- 失败信息
                        `cost` INTEGER NOT NULL, -- 耗时,毫秒
                        `status` BOOLEAN NOT NULL DEFAULT 0, -- 任务状态(0:开始,1:完成,2:停止,3:移除)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );
                    
                    CREATE INDEX idx_t_schedule_status_log_job_id ON t_schedule_status_log (`job_id`);
                    CREATE INDEX idx_t_schedule_status_log_uuid ON t_schedule_status_log (`uuid`);
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
            .execute_unprepared("DROP TABLE `t_schedule_status_log`")
            .await?;

        Ok(())
    }
}
