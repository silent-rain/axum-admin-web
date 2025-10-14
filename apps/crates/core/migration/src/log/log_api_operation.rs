//! API操作日志表
//! Entity: [`entity::log::LogApiOperation`]

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
                    `t_log_api_operation` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '自增ID',
                        `user_id` INT NULL DEFAULT 0 COMMENT '用户ID',
                        `username` VARCHAR(32) NULL DEFAULT '' COMMENT '用户名称',
                        `request_id` VARCHAR(36) NULL DEFAULT '' COMMENT '请求ID',
                        `status_code` INT NOT NULL COMMENT '请求状态码',
                        `method` VARCHAR(10) NOT NULL COMMENT '请求方法',
                        `path` VARCHAR(500) NOT NULL COMMENT '请求地址路径',
                        `content_type` VARCHAR(100) NOT NULL COMMENT 'Content-Type',
                        `query` VARCHAR(500) NULL DEFAULT '' COMMENT '请求参数',
                        `body` MEDIUMTEXT NULL COMMENT '请求体/响应体',
                        `remote_addr` VARCHAR(64) NULL DEFAULT '' COMMENT '请求IP',
                        `user_agent` VARCHAR(256) NULL DEFAULT '' COMMENT '用户代理',
                        `cost` SMALLINT NOT NULL COMMENT '耗时,毫秒',
                        `http_type` VARCHAR(10) NOT NULL COMMENT '请求类型:REQ/RESP',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT 'API操作日志表';

                    CREATE INDEX idx_user_id ON t_log_api_operation (`user_id`);
                    CREATE INDEX idx_username ON t_log_api_operation (`username`);
                    CREATE INDEX idx_request_id ON t_log_api_operation (`request_id`);
                    CREATE INDEX idx_status_code ON t_log_api_operation (`status_code`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS
                    "t_log_api_operation" (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" INTEGER DEFAULT 0,
                        "username" VARCHAR(32) DEFAULT '',
                        "request_id" VARCHAR(32) DEFAULT '',
                        "status_code" INTEGER NOT NULL,
                        "method" VARCHAR(10) NOT NULL,
                        "path" VARCHAR(500) NOT NULL,
                        "content_type" VARCHAR(100) NOT NULL,
                        "query" VARCHAR(500) DEFAULT '',
                        "body" TEXT,
                        "remote_addr" VARCHAR(64) DEFAULT '',
                        "user_agent" VARCHAR(256) DEFAULT '',
                        "cost" INT2 NOT NULL,
                        "http_type" VARCHAR(10) NOT NULL,
                        "desc" VARCHAR(200) DEFAULT '',
                        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_log_api_operation_user_id ON t_log_api_operation ("user_id");
                    CREATE INDEX idx_t_log_api_operation_username ON t_log_api_operation ("username");
                    CREATE INDEX idx_t_log_api_operation_request_id ON t_log_api_operation ("request_id");
                    CREATE INDEX idx_t_log_api_operation_status_code ON t_log_api_operation ("status_code");

                    COMMENT ON TABLE t_log_api_operation IS 'API操作日志表';
                    COMMENT ON COLUMN t_log_api_operation.id IS '自增ID';
                    COMMENT ON COLUMN t_log_api_operation.user_id IS '用户ID';
                    COMMENT ON COLUMN t_log_api_operation.username IS '用户名称';
                    COMMENT ON COLUMN t_log_api_operation.request_id IS '请求ID';
                    COMMENT ON COLUMN t_log_api_operation.status_code IS '请求状态码';
                    COMMENT ON COLUMN t_log_api_operation.method IS '请求方法';
                    COMMENT ON COLUMN t_log_api_operation.path IS '请求地址路径';
                    COMMENT ON COLUMN t_log_api_operation.content_type IS 'Content-Type';
                    COMMENT ON COLUMN t_log_api_operation.query IS '请求参数';
                    COMMENT ON COLUMN t_log_api_operation.body IS '请求体/响应体';
                    COMMENT ON COLUMN t_log_api_operation.remote_addr IS '请求IP';
                    COMMENT ON COLUMN t_log_api_operation.user_agent IS '用户代理';
                    COMMENT ON COLUMN t_log_api_operation.cost IS '耗时,毫秒';
                    COMMENT ON COLUMN t_log_api_operation.http_type IS '请求类型:REQ/RESP';
                    COMMENT ON COLUMN t_log_api_operation.desc IS '描述信息';
                    COMMENT ON COLUMN t_log_api_operation.created_at IS '创建时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_log_api_operation` ( -- API操作日志表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
                        `user_id` INTEGER DEFAULT 0, -- 用户ID
                        `username` TEXT DEFAULT '', -- 用户名称
                        `request_id` TEXT DEFAULT '', -- 请求ID
                        `status_code` INTEGER NOT NULL, -- 请求状态码
                        `method` TEXT NOT NULL, -- 请求方法
                        `path` TEXT NOT NULL, -- 请求地址路径
                        `content_type` TEXT NOT NULL, -- Content-Type
                        `query` TEXT DEFAULT '', -- 请求参数
                        `body` TEXT, -- 请求体/响应体
                        `remote_addr` TEXT DEFAULT '', -- 请求IP
                        `user_agent` TEXT DEFAULT '', -- 用户代理
                        `cost` INTEGER NOT NULL, -- 耗时,毫秒
                        `http_type` TEXT NOT NULL, -- 请求类型:REQ/RESP
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP -- 创建时间
                    );

                    CREATE INDEX idx_t_log_api_operation_user_id ON t_log_api_operation (`user_id`);
                    CREATE INDEX idx_t_log_api_operation_username ON t_log_api_operation (`username`);
                    CREATE INDEX idx_t_log_api_operation_request_id ON t_log_api_operation (`request_id`);
                    CREATE INDEX idx_t_log_api_operation_status_code ON t_log_api_operation (`status_code`);
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
            .execute_unprepared("DROP TABLE `t_log_api_operation;`")
            .await?;

        Ok(())
    }
}
