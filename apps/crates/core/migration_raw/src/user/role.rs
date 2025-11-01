//! 角色表
//! Entity: [`entity::user::Role`]

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
                    `t_user_role` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '角色ID',
                        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '角色名称',
                        `sort` INT NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '角色表';
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS 
                    "t_user_role" (
                        "id" SERIAL PRIMARY KEY,
                        "name" VARCHAR(20) UNIQUE NOT NULL,
                        "sort" INT NULL DEFAULT 0,
                        "desc" VARCHAR(200) NULL DEFAULT '',
                        "status" BOOL NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    COMMENT ON TABLE t_user_role IS '角色表';
                    COMMENT ON COLUMN t_user_role.id IS '角色ID';
                    COMMENT ON COLUMN t_user_role.name IS '角色名称';
                    COMMENT ON COLUMN t_user_role.sort IS '排序';
                    COMMENT ON COLUMN t_user_role.desc IS '描述信息';
                    COMMENT ON COLUMN t_user_role.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_user_role.created_at IS '创建时间';
                    COMMENT ON COLUMN t_user_role.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_user_role` ( -- 角色表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 角色ID
                        `name` TEXT UNIQUE NOT NULL, -- 角色名称
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );
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
            .execute_unprepared("DROP TABLE `t_user_role`")
            .await?;

        Ok(())
    }
}
