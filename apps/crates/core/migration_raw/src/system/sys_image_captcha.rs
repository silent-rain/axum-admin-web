//! 图片验证码表
//! Entity: [`entity::system::SysImageCaptcha`]

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
                    `t_sys_image_captcha` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '自增ID',
                        `captcha_id` VARCHAR(40) UNIQUE NOT NULL COMMENT '验证码ID',
                        `captcha` VARCHAR(10) NOT NULL COMMENT '验证码',
                        `data` MEDIUMBLOB NOT NULL COMMENT '图片数据, Base64编码',
                        `expire` SMALLINT NOT NULL DEFAULT 1 COMMENT '过期时间,秒',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:失效,true:有效)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '图片图片验证码表';

                    CREATE INDEX idx_captcha_id ON t_sys_image_captcha (`captcha_id`);
                    CREATE INDEX idx_status ON t_sys_image_captcha (`status`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_sys_image_captcha" (
                        "id" SERIAL PRIMARY KEY,
                        "captcha_id" VARCHAR(40) UNIQUE NOT NULL,
                        "captcha" VARCHAR(10) NOT NULL,
                        "data" BYTEA NOT NULL,
                        "expire" SMALLINT NOT NULL DEFAULT 1,
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_sys_image_captcha_captcha_id ON t_sys_image_captcha ("captcha_id");
                    CREATE INDEX idx_t_sys_image_captcha_status ON t_sys_image_captcha ("status");

                    COMMENT ON TABLE t_sys_image_captcha IS '图片验证码表';
                    COMMENT ON COLUMN t_sys_image_captcha.id IS '自增ID';
                    COMMENT ON COLUMN t_sys_image_captcha.captcha_id IS '验证码ID';
                    COMMENT ON COLUMN t_sys_image_captcha.captcha IS '验证码';
                    COMMENT ON COLUMN t_sys_image_captcha.data IS '图片数据, Base64编码';
                    COMMENT ON COLUMN t_sys_image_captcha.expire IS '过期时间,秒';
                    COMMENT ON COLUMN t_sys_image_captcha.status IS '状态(false:失效,true:有效)';
                    COMMENT ON COLUMN t_sys_image_captcha.created_at IS '创建时间';
                    COMMENT ON COLUMN t_sys_image_captcha.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_sys_image_captcha` ( -- 图片验证码表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
                        `captcha_id` TEXT UNIQUE NOT NULL, -- 验证码ID
                        `captcha` TEXT NOT NULL, -- 验证码
                        `data` BLOB NOT NULL, -- 图片数据, Base64编码
                        `expire` SMALLINT NOT NULL DEFAULT 1, -- 过期时间,秒
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:失效,true:有效)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );
                    
                    CREATE INDEX idx_t_sys_image_captcha_captcha_id ON t_sys_image_captcha (`captcha_id`);
                    CREATE INDEX idx_t_sys_image_captcha_status ON t_sys_image_captcha (`status`);
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
            .execute_unprepared("DROP TABLE `t_sys_image_captcha`")
            .await?;

        Ok(())
    }
}
