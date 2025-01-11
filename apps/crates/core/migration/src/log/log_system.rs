//! 系统日志表
//! Entity: [`entity::log::LogSystem`]

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
                    `t_log_system` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
                        `user_id` INT(20) NULL DEFAULT 0 COMMENT '请求用户ID',
                        `username` VARCHAR(32) NULL DEFAULT '' COMMENT '用户名称',
                        `name` VARCHAR(50) NOT NULL COMMENT '日志记录器名称',
                        `span_pid` INT(20) NULL DEFAULT 0 COMMENT 'Span Parent Id',
                        `span_id` INT(20) NULL DEFAULT 0 COMMENT 'Span Id',
                        `module_path` VARCHAR(100) NULL DEFAULT '' COMMENT '模块路径',
                        `target` VARCHAR(100) NULL DEFAULT '' COMMENT '描述发生此元数据所描述的跨度或事件的系统部分',
                        `file` VARCHAR(500) NULL DEFAULT '' COMMENT '文件',
                        `line` INT(10) UNSIGNED NULL DEFAULT 0 COMMENT '报错行数',
                        `level` VARCHAR(10) NOT NULL DEFAULT '' COMMENT '日志级别',
                        `kind` VARCHAR(10) NOT NULL DEFAULT '' COMMENT '事件类型',
                        `is_event` BOOL NOT NULL DEFAULT false COMMENT '是否为事件',
                        `is_span` BOOL NOT NULL DEFAULT false COMMENT '是否为 span',
                        `fields` VARCHAR(500) NULL DEFAULT '' COMMENT '日志字段名称列表',
                        `field_data` TEXT NULL COMMENT 'fields 日志数据集',
                        `message` TEXT NULL COMMENT '日志信息',
                        `code` INT(10) NULL DEFAULT 0 COMMENT '业务误码',
                        `code_msg` VARCHAR(500) NULL DEFAULT '' COMMENT '业务误码信息',
                        `stack` TEXT NULL COMMENT '堆栈信息',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB AUTO_INCREMENT = 1485 DEFAULT CHARSET = utf8mb4 COMMENT = '系统日志';

                    CREATE INDEX idx_user_id ON t_log_system (`user_id`);
                    CREATE INDEX idx_username ON t_log_system (`username`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS
                    "t_log_system" (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" BIGINT DEFAULT 0,
                        "username" VARCHAR(32) DEFAULT '',
                        "name" VARCHAR(50) NOT NULL,
                        "span_pid" BIGINT DEFAULT 0,
                        "span_id" BIGINT DEFAULT 0,
                        "module_path" VARCHAR(100) DEFAULT '',
                        "target" VARCHAR(100) DEFAULT '',
                        "file" VARCHAR(500) DEFAULT '',
                        "line" BIGINT DEFAULT 0,
                        "level" VARCHAR(10) NOT NULL DEFAULT '',
                        "kind" VARCHAR(10) NOT NULL DEFAULT '',
                        "is_event" BOOL NOT NULL DEFAULT FALSE,
                        "is_span" BOOL NOT NULL DEFAULT FALSE,
                        "fields" VARCHAR(500) DEFAULT '',
                        "field_data" TEXT,
                        "message" TEXT,
                        "code" INTEGER DEFAULT 0,
                        "code_msg" VARCHAR(500) DEFAULT '',
                        "stack" TEXT,
                        "desc" VARCHAR(200) DEFAULT '',
                        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_log_system_user_id ON t_log_system ("user_id");
                    CREATE INDEX idx_t_log_system_username ON t_log_system ("username");

                    COMMENT ON TABLE t_log_system IS '系统日志';
                    COMMENT ON COLUMN t_log_system.id IS '自增ID';
                    COMMENT ON COLUMN t_log_system.user_id IS '请求用户ID';
                    COMMENT ON COLUMN t_log_system.username IS '用户名称';
                    COMMENT ON COLUMN t_log_system.name IS '日志记录器名称';
                    COMMENT ON COLUMN t_log_system.span_pid IS 'Span Parent Id';
                    COMMENT ON COLUMN t_log_system.span_id IS 'Span Id';
                    COMMENT ON COLUMN t_log_system.module_path IS '模块路径';
                    COMMENT ON COLUMN t_log_system.target IS '描述发生此元数据所描述的跨度或事件的系统部分';
                    COMMENT ON COLUMN t_log_system.file IS '文件';
                    COMMENT ON COLUMN t_log_system.line IS '报错行数';
                    COMMENT ON COLUMN t_log_system.level IS '日志级别';
                    COMMENT ON COLUMN t_log_system.kind IS '事件类型';
                    COMMENT ON COLUMN t_log_system.is_event IS '是否为事件';
                    COMMENT ON COLUMN t_log_system.is_span IS '是否为 span';
                    COMMENT ON COLUMN t_log_system.fields IS '日志字段名称列表';
                    COMMENT ON COLUMN t_log_system.field_data IS 'fields 日志数据集';
                    COMMENT ON COLUMN t_log_system.message IS '日志信息';
                    COMMENT ON COLUMN t_log_system.code IS '业务误码';
                    COMMENT ON COLUMN t_log_system.code_msg IS '业务误码信息';
                    COMMENT ON COLUMN t_log_system.stack IS '堆栈信息';
                    COMMENT ON COLUMN t_log_system.desc IS '描述信息';
                    COMMENT ON COLUMN t_log_system.created_at IS '创建时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_log_system` ( -- 系统日志
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
                        `user_id` INTEGER DEFAULT 0, -- 请求用户ID
                        `username` TEXT DEFAULT '', -- 用户名称
                        `name` TEXT NOT NULL, -- 日志记录器名称
                        `span_pid` INTEGER DEFAULT 0, -- Span Parent Id
                        `span_id` INTEGER DEFAULT 0, -- Span Id
                        `module_path` TEXT DEFAULT '', -- 模块路径
                        `target` TEXT DEFAULT '', -- 描述发生此元数据所描述的跨度或事件的系统部分
                        `file` TEXT DEFAULT '', -- 文件
                        `line` INTEGER UNSIGNED DEFAULT 0, -- 报错行数
                        `level` TEXT NOT NULL DEFAULT '', -- 日志级别
                        `kind` TEXT NOT NULL DEFAULT '', -- 事件类型
                        `is_event` BOOLEAN NOT NULL DEFAULT false, -- 是否为事件
                        `is_span` BOOLEAN NOT NULL DEFAULT false, -- 是否为 span
                        `fields` TEXT DEFAULT '', -- 日志字段名称列表
                        `field_data` TEXT, -- fields 日志数据集
                        `message` TEXT, -- 日志信息
                        `code` INTEGER DEFAULT 0, -- 业务误码
                        `code_msg` TEXT DEFAULT '', -- 业务误码信息
                        `stack` TEXT, -- 堆栈信息
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP -- 创建时间
                    );

                    CREATE INDEX idx_t_log_system_user_id ON t_log_system (`user_id`);
                    CREATE INDEX idx_t_log_system_username ON t_log_system (`username`);
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
            .execute_unprepared("DROP TABLE `t_log_system;`")
            .await?;

        Ok(())
    }
}
