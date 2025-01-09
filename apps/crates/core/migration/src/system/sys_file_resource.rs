//! 文件资源表
//! Entity: [`entity::system::SysFileResource`]

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
                    `t_sys_file_resource` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '图片ID',
                        `name` VARCHAR(32) NOT NULL COMMENT '图片名称',
                        `hash` VARCHAR(32) UNIQUE NOT NULL COMMENT '图片HASH值',
                        `data` MEDIUMBLOB NOT NULL COMMENT '图片数据, Base64编码',
                        `extension` VARCHAR(10) NOT NULL COMMENT '图片文件扩展名, 如svg, png',
                        `size` INT(10) NOT NULL COMMENT '图片文件大小，单位为字节',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '图片资源表';

                    CREATE INDEX idx_name ON t_sys_file_resource (`name`);
                    CREATE INDEX idx_hash ON t_sys_file_resource (`hash`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_sys_file_resource" (
                        "id" SERIAL PRIMARY KEY,
                        "name" VARCHAR(32) NOT NULL,
                        "hash" VARCHAR(32) UNIQUE NOT NULL,
                        "data" BYTEA NOT NULL,
                        "extension" VARCHAR(10) NOT NULL,
                        "size" INTEGER NOT NULL,
                        "desc" VARCHAR(200) DEFAULT '',
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_sys_file_resource_name ON t_sys_file_resource ("name");
                    CREATE INDEX idx_t_sys_file_resource_hash ON t_sys_file_resource ("hash");

                    COMMENT ON TABLE t_sys_file_resource IS '图片资源表';
                    COMMENT ON COLUMN t_sys_file_resource.id IS '图片ID';
                    COMMENT ON COLUMN t_sys_file_resource.name IS '图片名称';
                    COMMENT ON COLUMN t_sys_file_resource.hash IS '图片HASH值';
                    COMMENT ON COLUMN t_sys_file_resource.data IS '图片数据, Base64编码';
                    COMMENT ON COLUMN t_sys_file_resource.extension IS '图片文件扩展名, 如svg, png';
                    COMMENT ON COLUMN t_sys_file_resource.size IS '图片文件大小，单位为字节';
                    COMMENT ON COLUMN t_sys_file_resource.desc IS '描述信息';
                    COMMENT ON COLUMN t_sys_file_resource.created_at IS '创建时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_sys_file_resource` ( -- 图片资源表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 图片ID
                        `name` TEXT NOT NULL, -- 图片名称
                        `hash` TEXT UNIQUE NOT NULL, -- 图片HASH值
                        `data` BLOB NOT NULL, -- 图片数据, Base64编码
                        `extension` TEXT NOT NULL, -- 图片文件扩展名, 如svg, png
                        `size` INTEGER NOT NULL, -- 图片文件大小，单位为字节
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 创建时间
                    );
                    
                    CREATE INDEX idx_t_sys_file_resource_name ON t_sys_file_resource (`name`);
                    CREATE INDEX idx_t_sys_file_resource_hash ON t_sys_file_resource (`hash`);
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
            .execute_unprepared("DROP TABLE `t_sys_file_resource`")
            .await?;

        Ok(())
    }
}
