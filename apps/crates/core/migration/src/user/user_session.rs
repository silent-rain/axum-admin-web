//! 用户session表
//! Entity: [`entity::user::UserSession`]

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
                    `t_user_session` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '自增ID',
                        `user_id` INT NOT NULL COMMENT '用户ID',
                        `username` VARCHAR(32) NOT NULL COMMENT '用户名称',
                        `session_id` VARCHAR(40) UNIQUE NOT NULL COMMENT '用户会话ID',
                        `expiry_date` DATETIME NOT NULL COMMENT '过期时间',
                        `data` MEDIUMBLOB NOT NULL COMMENT '元数据',
                        `desc` VARCHAR(200) DEFAULT '' COMMENT '描述信息',
                        `status` BOOL NOT NULL DEFAULT FALSE COMMENT '登录状态是否有效(false:无效,true:有效)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '用户session表';

                    CREATE INDEX idx_t_user_session_user_id ON t_user_session (`user_id`);
                    CREATE INDEX idx_t_user_session_status ON t_user_session (`status`);
                    CREATE INDEX idx_t_user_session_expiry_date ON t_user_session (`expiry_date`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS 
                    "t_user_session" (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" INT NOT NULL,
                        "username" VARCHAR(32) NOT NULL,
                        "session_id" VARCHAR(40) UNIQUE NOT NULL,
                        "expiry_date" TIMESTAMP NOT NULL,
                        "data" BYTEA NOT NULL,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" BOOL NULL DEFAULT false,
                        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_user_session_user_id ON t_user_session (`user_id`);
                    CREATE INDEX idx_t_user_session_status ON t_user_session (`status`);
                    CREATE INDEX idx_t_user_session_expiry_date ON t_user_session (`expiry_date`);

                    COMMENT ON TABLE t_user_session IS '用户session表';
                    COMMENT ON COLUMN t_user_session.id IS '自增ID';
                    COMMENT ON COLUMN t_user_session.user_id IS '用户ID';
                    COMMENT ON COLUMN t_user_session.username IS '用户名称';
                    COMMENT ON COLUMN t_user_session.session_id IS '用户会话ID';
                    COMMENT ON COLUMN t_user_session.expiry_date IS '过期时间';
                    COMMENT ON COLUMN t_user_session.data IS '元数据';
                    COMMENT ON COLUMN t_user_session.desc IS '描述信息';
                    COMMENT ON COLUMN t_user_session.status IS '登录状态是否有效(false:无效,true:有效)';
                    COMMENT ON COLUMN t_user_session.created_at IS '创建时间';
                    COMMENT ON COLUMN t_user_session.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_user_session` ( -- 用户session表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
                        `user_id` INTEGER NOT NULL, -- 用户ID
                        `username` VARCHAR(32) NOT NULL, -- 用户名称
                        `session_id` VARCHAR(40) UNIQUE NOT NULL, -- 用户会话ID
                        `expiry_date` TIMESTAMP NOT NULL, -- 过期时间
                        `data` BLOB NOT NULL, -- 元数据
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `status` BOOLEAN NOT NULL DEFAULT false, -- 登录状态是否有效(false:无效,true:有效)
                        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_user_session_user_id ON t_user_session (`user_id`);
                    CREATE INDEX idx_t_user_session_status ON t_user_session (`status`);
                    CREATE INDEX idx_t_user_session_expiry_date ON t_user_session (`expiry_date`);
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
            .execute_unprepared("DROP TABLE `t_user_session`")
            .await?;

        Ok(())
    }
}
