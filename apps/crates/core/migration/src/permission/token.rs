//! 令牌表
//! Entity: [`entity::permission::Token`]

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
                    `t_perm_token` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '令牌ID',
                        `user_id` INT NOT NULL COMMENT '用户ID',
                        `token` VARCHAR(50) UNIQUE NOT NULL COMMENT '令牌',
                        `passphrase` VARCHAR(20) NOT NULL COMMENT '口令',
                        `permission` VARCHAR(20) NOT NULL COMMENT '权限范围:GET,POST,PUT,DELETE',
                        `expire` DATETIME NOT NULL COMMENT '授权到期时间',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '令牌表';

                    CREATE INDEX idx_user_id ON t_perm_token (`user_id`);
                    CREATE INDEX idx_token ON t_perm_token (`token`);
                    CREATE INDEX idx_passphrase ON t_perm_token (`passphrase`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_perm_token" (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" INTEGER NOT NULL,
                        "token" VARCHAR(50) UNIQUE NOT NULL,
                        "passphrase" VARCHAR(20) NOT NULL,
                        "permission" VARCHAR(20) NOT NULL,
                        "expire" TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_perm_token_user_id ON t_perm_token ("user_id");
                    CREATE INDEX idx_t_perm_token_token ON t_perm_token ("token");
                    CREATE INDEX idx_t_perm_token_passphrase ON t_perm_token ("passphrase");

                    COMMENT ON TABLE t_perm_token IS '令牌表';
                    COMMENT ON COLUMN t_perm_token.id IS '令牌ID';
                    COMMENT ON COLUMN t_perm_token.user_id IS '用户ID';
                    COMMENT ON COLUMN t_perm_token.token IS '令牌';
                    COMMENT ON COLUMN t_perm_token.passphrase IS '口令';
                    COMMENT ON COLUMN t_perm_token.permission IS '权限范围:GET,POST,PUT,DELETE';
                    COMMENT ON COLUMN t_perm_token.expire IS '授权到期时间';
                    COMMENT ON COLUMN t_perm_token.desc IS '描述信息';
                    COMMENT ON COLUMN t_perm_token.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_perm_token.created_at IS '创建时间';
                    COMMENT ON COLUMN t_perm_token.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_perm_token` ( -- 令牌表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 令牌ID
                        `user_id` INTEGER NOT NULL, -- 用户ID
                        `token` VARCHAR(50) UNIQUE NOT NULL, -- 令牌
                        `passphrase` VARCHAR(20) NOT NULL, -- 口令
                        `permission` VARCHAR(20) NOT NULL, -- 权限范围:GET,POST,PUT,DELETE
                        `expire` TIMESTAMP NOT NULL, -- 授权到期时间
                        `desc` VARCHAR(200) DEFAULT '', -- 描述信息
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_perm_token_user_id ON t_perm_token (`user_id`);
                    CREATE INDEX idx_t_perm_token_token ON t_perm_token (`token`);
                    CREATE INDEX idx_t_perm_token_passphrase ON t_perm_token (`passphrase`);
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
            .execute_unprepared("DROP TABLE `t_perm_token;`")
            .await?;

        Ok(())
    }
}
