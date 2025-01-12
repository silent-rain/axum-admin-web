//! 任务调度事件日志表
//! Entity: [`entity::schedule::ScheduleEventLog`]

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
                    `t_schedule_event_log` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '事件日志ID',
                        `job_id` INT NOT NULL COMMENT '任务ID',
                        `uuid` VARCHAR(50) NOT NULL COMMENT '任务调度ID',
                        `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务状态(0:开始,1:完成,2:停止,3:移除)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
                        PRIMARY KEY (`id`) USING BTREE
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '任务调度事件日志表';

                    CREATE INDEX idx_job_id ON t_schedule_event_log (`job_id`);
                    CREATE INDEX idx_uuid ON t_schedule_event_log (`uuid`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared( 
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_schedule_event_log" (
                        "id" SERIAL PRIMARY KEY,
                        "job_id" INTEGER NOT NULL,
                        "uuid" VARCHAR(50) NOT NULL,
                        "status" char NOT NULL DEFAULT 1,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_schedule_event_log_job_id ON t_schedule_event_log ("job_id");
                    CREATE INDEX idx_t_schedule_event_log_uuid ON t_schedule_event_log ("uuid");

                    COMMENT ON TABLE t_schedule_event_log IS '任务调度事件日志表';
                    COMMENT ON COLUMN t_schedule_event_log.id IS '事件日志ID';
                    COMMENT ON COLUMN t_schedule_event_log.job_id IS '任务ID';
                    COMMENT ON COLUMN t_schedule_event_log.uuid IS '任务调度ID';
                    COMMENT ON COLUMN t_schedule_event_log.status IS '任务状态(0:开始,1:完成,2:停止,3:移除)';
                    COMMENT ON COLUMN t_schedule_event_log.created_at IS '创建时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_schedule_event_log` ( -- 任务调度事件日志表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 事件日志ID
                        `job_id` INTEGER NOT NULL, -- 任务ID
                        `uuid` TEXT NOT NULL, -- 任务调度ID
                        `status` TINYINT(1) NOT NULL DEFAULT 0, -- 任务状态(0:开始,1:完成,2:停止,3:移除)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 创建时间
                    );
                    
                    CREATE INDEX idx_t_schedule_event_log_job_id ON t_schedule_event_log (`job_id`);
                    CREATE INDEX idx_t_schedule_event_log_uuid ON t_schedule_event_log (`uuid`);
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
            .execute_unprepared("DROP TABLE `t_schedule_event_log`")
            .await?;

        Ok(())
    }
}
