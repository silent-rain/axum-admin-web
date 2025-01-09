//! 用户邮箱表
//! Entity: [`entity::user::Email`]

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
                    `t_user_email` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '邮箱ID',
                        `user_id` INT(10) UNIQUE NOT NULL COMMENT '用户ID',
                        `email` VARCHAR(50) UNIQUE NOT NULL COMMENT '邮箱',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '用户邮箱';
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS 
                    "t_user_email" (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" INT UNIQUE NOT NULL,
                        "email" VARCHAR(50) UNIQUE NOT NULL,
                        "desc" VARCHAR(200) DEFAULT '',
                        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    COMMENT ON TABLE t_user_email IS '用户邮箱';
                    COMMENT ON COLUMN t_user_email.id IS '邮箱ID';
                    COMMENT ON COLUMN t_user_email.user_id IS '用户ID';
                    COMMENT ON COLUMN t_user_email.email IS '邮箱';
                    COMMENT ON COLUMN t_user_email.desc IS '描述信息';
                    COMMENT ON COLUMN t_user_email.created_at IS '创建时间';
                    COMMENT ON COLUMN t_user_email.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_user_email` (  -- 用户邮箱
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 邮箱ID
                        `user_id` INTEGER UNIQUE NOT NULL, -- 用户ID
                        `email` TEXT UNIQUE NOT NULL, -- 邮箱
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
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
            .execute_unprepared("DROP TABLE `t_user_email`")
            .await?;

        Ok(())
    }
}
