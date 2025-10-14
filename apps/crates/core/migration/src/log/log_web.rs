//! WEB日志表
//! Entity: [`entity::log::LogWeb`]

use sea_orm::{ConnectionTrait, DatabaseBackend, DeriveMigrationName};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

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
                    `t_log_web` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '日志ID',
                        `user_id` INT NULL DEFAULT 0 COMMENT '用户ID',
                        `username` VARCHAR(32) NULL DEFAULT '' COMMENT '用户名称',
                        `request_id` VARCHAR(36) NULL DEFAULT '' COMMENT '请求ID',
                        `os_type` TINYINT(1) NOT NULL COMMENT '终端类型(0:未知, 1:安卓, 2:IOS, 3:WEB)',
                        `error_type` TINYINT(1) NOT NULL COMMENT '错误类型(1:接口报错, 2:代码报错)',
                        `level` VARCHAR(10) NOT NULL COMMENT '日志级别',
                        `caller_line` VARCHAR(100) NOT NULL COMMENT '日发生位置',
                        `url` VARCHAR(500) NULL COMMENT '请求地址',
                        `msg` TEXT NULL COMMENT '日志消息',
                        `stack` TEXT NULL COMMENT '堆栈信息',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT 'WEB日志表';

                    CREATE INDEX idx_user_id ON t_log_web (`user_id`);
                    CREATE INDEX idx_username ON t_log_web (`username`);
                    CREATE INDEX idx_request_id ON t_log_web (`request_id`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS
                    "t_log_web" (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" INTEGER DEFAULT 0,
                        "username" VARCHAR(32) DEFAULT '',
                        "request_id" VARCHAR(36) DEFAULT '',
                        "os_type" char NOT NULL,
                        "error_type" char NOT NULL,
                        "level" VARCHAR(10) NOT NULL,
                        "caller_line" VARCHAR(100) NOT NULL,
                        "url" VARCHAR(500),
                        "msg" TEXT,
                        "stack" TEXT,
                        "desc" VARCHAR(200) DEFAULT '',
                        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_log_web_user_id ON t_log_web ("user_id");
                    CREATE INDEX idx_t_log_web_username ON t_log_web ("username");
                    CREATE INDEX idx_t_log_web_request_id ON t_log_web ("request_id");

                    COMMENT ON TABLE t_log_web IS 'WEB日志表';
                    COMMENT ON COLUMN t_log_web.id IS '日志ID';
                    COMMENT ON COLUMN t_log_web.user_id IS '用户ID';
                    COMMENT ON COLUMN t_log_web.username IS '用户名称';
                    COMMENT ON COLUMN t_log_web.request_id IS '请求ID';
                    COMMENT ON COLUMN t_log_web.os_type IS '终端类型(0:未知, 1:安卓, 2:IOS, 3:WEB)';
                    COMMENT ON COLUMN t_log_web.error_type IS '错误类型(1:接口报错, 2:代码报错)';
                    COMMENT ON COLUMN t_log_web.level IS '日志级别';
                    COMMENT ON COLUMN t_log_web.caller_line IS '日发生位置';
                    COMMENT ON COLUMN t_log_web.url IS '请求地址';
                    COMMENT ON COLUMN t_log_web.msg IS '日志消息';
                    COMMENT ON COLUMN t_log_web.stack IS '堆栈信息';
                    COMMENT ON COLUMN t_log_web.desc IS '描述信息';
                    COMMENT ON COLUMN t_log_web.created_at IS '创建时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_log_web` ( -- WEB日志表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 日志ID
                        `user_id` INTEGER DEFAULT 0, -- 用户ID
                        `username` TEXT DEFAULT '', -- 用户名称
                        `request_id` TEXT DEFAULT '', -- 请求ID
                        `os_type` INTEGER NOT NULL, -- 终端类型(0:未知, 1:安卓, 2:IOS, 3:WEB)
                        `error_type` INTEGER NOT NULL, -- 错误类型(1:接口报错, 2:代码报错)
                        `level` TEXT NOT NULL, -- 日志级别
                        `caller_line` TEXT NOT NULL, -- 日发生位置
                        `url` TEXT, -- 请求地址
                        `msg` TEXT, -- 日志消息
                        `stack` TEXT, -- 堆栈信息
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP -- 创建时间
                    );

                    CREATE INDEX idx_t_log_web_user_id ON t_log_web (`user_id`);
                    CREATE INDEX idx_t_log_web_username ON t_log_web (`username`);
                    CREATE INDEX idx_t_log_web_request_id ON t_log_web (`request_id`);
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
            .execute_unprepared("DROP TABLE `t_log_web;`")
            .await?;

        Ok(())
    }
}
