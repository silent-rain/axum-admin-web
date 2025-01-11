//! 配置表
//! Entity: [`entity::system::SysConfig`]

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
                    `t_sys_config` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '配置ID',
                        `pid` INT(11) DEFAULT 0 COMMENT '父节点ID',
                        `name` VARCHAR(64) NOT NULL COMMENT '配置名称',
                        `code` VARCHAR(64) UNIQUE NOT NULL COMMENT '配置编码(英文)',
                        `value` TEXT NULL COMMENT '配置值',
                        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) DEFAULT '' COMMENT '配置描述',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '配置表';

                    CREATE INDEX idx_pid ON t_sys_config (`pid`);
                    CREATE INDEX idx_name ON t_sys_config (`name`);
                    CREATE INDEX idx_code ON t_sys_config (`code`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_sys_config" (
                        "id" SERIAL PRIMARY KEY,
                        "pid" INTEGER DEFAULT 0,
                        "name" VARCHAR(64) NOT NULL,
                        "code" VARCHAR(64) UNIQUE NOT NULL,
                        "value" TEXT,
                        "sort" INTEGER DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_sys_config_pid ON t_sys_config ("pid");
                    CREATE INDEX idx_t_sys_config_name ON t_sys_config ("name");
                    CREATE INDEX idx_t_sys_config_code ON t_sys_config ("code");

                    COMMENT ON TABLE t_sys_config IS '配置表';
                    COMMENT ON COLUMN t_sys_config.id IS '配置ID';
                    COMMENT ON COLUMN t_sys_config.pid IS '父节点ID';
                    COMMENT ON COLUMN t_sys_config.name IS '配置名称';
                    COMMENT ON COLUMN t_sys_config.code IS '配置编码(英文)';
                    COMMENT ON COLUMN t_sys_config.value IS '配置值';
                    COMMENT ON COLUMN t_sys_config.sort IS '排序';
                    COMMENT ON COLUMN t_sys_config.desc IS '配置描述';
                    COMMENT ON COLUMN t_sys_config.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_sys_config.created_at IS '创建时间';
                    COMMENT ON COLUMN t_sys_config.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_sys_config` ( -- 配置表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 配置ID
                        `pid` INTEGER DEFAULT 0, -- 父节点ID
                        `name` TEXT NOT NULL, -- 配置名称
                        `code` TEXT UNIQUE NOT NULL, -- 配置编码(英文)
                        `value` TEXT, -- 配置值
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` TEXT DEFAULT '', -- 配置描述
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_sys_config_pid ON t_sys_config (`pid`);
                    CREATE INDEX idx_t_sys_config_name ON t_sys_config (`name`);
                    CREATE INDEX idx_t_sys_config_code ON t_sys_config (`code`);
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
            .execute_unprepared("DROP TABLE `t_sys_config`")
            .await?;

        Ok(())
    }
}
