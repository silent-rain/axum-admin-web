//! 文件资源表
//! Entity: [`entity::system::SysFileResource`]

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
                    `t_sys_file_resource` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '文件ID',
                        `file_name` VARCHAR(32) NOT NULL COMMENT '文件名称',
                        `hash` VARCHAR(32) UNIQUE NOT NULL COMMENT '文件HASH值',
                        `data` MEDIUMBLOB NOT NULL COMMENT '文件数据, Base64编码',
                        `extension` VARCHAR(20) NOT NULL COMMENT '文件文件扩展名, 如svg, png',
                        `content_type` VARCHAR(20) NOT NULL COMMENT '内容类型, text/html',
                        `size` INT NOT NULL COMMENT '文件文件大小，单位为字节',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '文件资源表';

                    CREATE INDEX idx_t_sys_file_resource_file_name ON t_sys_file_resource (`file_name`);
                    CREATE INDEX idx_t_sys_file_resource_hash ON t_sys_file_resource (`hash`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_sys_file_resource" (
                        "id" SERIAL PRIMARY KEY,
                        "file_name" VARCHAR(32) NOT NULL,
                        "hash" VARCHAR(32) UNIQUE NOT NULL,
                        "data" BYTEA NOT NULL,
                        "extension" VARCHAR(20) NOT NULL,
                        "content_type" VARCHAR(20) NOT NULL,
                        "size" INTEGER NOT NULL,
                        "desc" VARCHAR(200) DEFAULT '',
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_sys_file_resource_file_name ON t_sys_file_resource ("file_name");
                    CREATE INDEX idx_t_sys_file_resource_hash ON t_sys_file_resource ("hash");

                    COMMENT ON TABLE t_sys_file_resource IS '文件资源表';
                    COMMENT ON COLUMN t_sys_file_resource.id IS '文件ID';
                    COMMENT ON COLUMN t_sys_file_resource.file_name IS '文件名称';
                    COMMENT ON COLUMN t_sys_file_resource.hash IS '文件HASH值';
                    COMMENT ON COLUMN t_sys_file_resource.data IS '文件数据, Base64编码';
                    COMMENT ON COLUMN t_sys_file_resource.extension IS '文件文件扩展名, 如svg, png';
                    COMMENT ON COLUMN t_sys_file_resource.content_type IS '内容类型, text/html';
                    COMMENT ON COLUMN t_sys_file_resource.size IS '文件文件大小，单位为字节';
                    COMMENT ON COLUMN t_sys_file_resource.desc IS '描述信息';
                    COMMENT ON COLUMN t_sys_file_resource.created_at IS '创建时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_sys_file_resource` ( -- 文件资源表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 文件ID
                        `file_name` TEXT NOT NULL, -- 文件名称
                        `hash` TEXT UNIQUE NOT NULL, -- 文件HASH值
                        `data` BLOB NOT NULL, -- 文件数据, Base64编码
                        `extension` TEXT NOT NULL, -- 文件文件扩展名, 如svg, png
                        `content_type` TEXT NOT NULL, -- 内容类型, text/html
                        `size` INTEGER NOT NULL, -- 文件文件大小，单位为字节
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 创建时间
                    );
                    
                    CREATE INDEX idx_t_sys_file_resource_file_name ON t_sys_file_resource (`file_name`);
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
