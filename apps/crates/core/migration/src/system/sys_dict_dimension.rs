//! 字典维度表
//! Entity: [`entity::system::SysDictDimension`]

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
                    `t_sys_dict_dimension` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '字典维度ID',
                        `name` VARCHAR(64) UNIQUE NOT NULL COMMENT '字典维度名称',
                        `code` VARCHAR(64) UNIQUE NOT NULL COMMENT '字典维度编码',
                        `sort` INT NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '字典维度表';

                    CREATE INDEX idx_name ON t_sys_dict_dimension (`name`);
                    CREATE INDEX idx_code ON t_sys_dict_dimension (`code`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_sys_dict_dimension" (
                        "id" SERIAL PRIMARY KEY,
                        "name" VARCHAR(64) UNIQUE NOT NULL,
                        "code" VARCHAR(64) UNIQUE NOT NULL,
                        "sort" INTEGER DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_sys_dict_dimension_name ON t_sys_dict_dimension ("name");
                    CREATE INDEX idx_t_sys_dict_code ON t_sys_dict_dimension ("code");

                    COMMENT ON TABLE t_sys_dict_dimension IS '字典维度表';
                    COMMENT ON COLUMN t_sys_dict_dimension.id IS '字典维度ID';
                    COMMENT ON COLUMN t_sys_dict_dimension.name IS '字典维度名称';
                    COMMENT ON COLUMN t_sys_dict_dimension.code IS '字典维度编码';
                    COMMENT ON COLUMN t_sys_dict_dimension.sort IS '排序';
                    COMMENT ON COLUMN t_sys_dict_dimension.desc IS '描述信息';
                    COMMENT ON COLUMN t_sys_dict_dimension.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_sys_dict_dimension.created_at IS '创建时间';
                    COMMENT ON COLUMN t_sys_dict_dimension.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_sys_dict_dimension` ( -- 字典维度表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 字典维度ID
                        `name` TEXT UNIQUE NOT NULL, -- 字典维度名称
                        `code` TEXT UNIQUE NOT NULL, -- 字典维度编码
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` TEXT DEFAULT '', -- 描述信息
                         `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );
                    
                    CREATE INDEX idx_t_sys_dict_dimension_name ON t_sys_dict_dimension (`name`);
                    CREATE INDEX idx_t_sys_dict_code ON t_sys_dict_dimension (`code`);
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
            .execute_unprepared("DROP TABLE `t_sys_dict_dimension`")
            .await?;

        Ok(())
    }
}
