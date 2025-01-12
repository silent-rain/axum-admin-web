//! 用户登录日志表
//! Entity: [`entity::user::UserLoginLog`]

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
                    `t_user_login_log` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '自增ID',
                        `user_id` INT NOT NULL COMMENT '用户ID',
                        `username` VARCHAR(32) NOT NULL COMMENT '用户名称',
                        `token` VARCHAR(300) NULL DEFAULT '' COMMENT '登陆令牌',
                        `remote_addr` VARCHAR(64) NULL DEFAULT '' COMMENT '登录IP',
                        `user_agent` VARCHAR(256) NULL DEFAULT '' COMMENT '用户代理',
                        `device` VARCHAR(20) NULL DEFAULT '' COMMENT '设备',
                        `system` VARCHAR(20) NULL DEFAULT '' COMMENT '系统',
                        `browser` VARCHAR(20) NULL DEFAULT '' COMMENT '浏览器',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `login_status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '登录状态(0:登陆成功,1:登陆失败,2:已禁用,3:登出)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '用户登录日志表';

                    CREATE INDEX idx_user_id ON t_user_login_log (`user_id`);
                    CREATE INDEX idx_username ON t_user_login_log (`username`);
                    CREATE INDEX idx_token ON t_user_login_log (`token`);
                    CREATE INDEX idx_login_status ON t_user_login_log (`login_status`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS 
                    "t_user_login_log" (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" INTEGER NOT NULL,
                        "username" VARCHAR(32) NOT NULL,
                        "token" VARCHAR(300) DEFAULT '',
                        "remote_addr" VARCHAR(64) DEFAULT '',
                        "user_agent" VARCHAR(256) DEFAULT '',
                        "device" VARCHAR(20) DEFAULT '',
                        "system" VARCHAR(20) DEFAULT '',
                        "browser" VARCHAR(20) DEFAULT '',
                        "desc" VARCHAR(200) DEFAULT '',
                        "login_status" char NOT NULL DEFAULT 0,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_user_login_log_user_id ON t_user_login_log ("user_id");
                    CREATE INDEX idx_t_user_login_log_username ON t_user_login_log ("username");
                    CREATE INDEX idx_t_user_login_log_token ON t_user_login_log ("token");
                    CREATE INDEX idx_t_user_login_log_login_status ON t_user_login_log ("login_status");

                    COMMENT ON TABLE t_user_login_log IS '用户登录日志表';
                    COMMENT ON COLUMN t_user_login_log.id IS '自增ID';
                    COMMENT ON COLUMN t_user_login_log.user_id IS '用户ID';
                    COMMENT ON COLUMN t_user_login_log.username IS '用户名称';
                    COMMENT ON COLUMN t_user_login_log.token IS '登录令牌';
                    COMMENT ON COLUMN t_user_login_log.remote_addr IS '登录IP';
                    COMMENT ON COLUMN t_user_login_log.user_agent IS '用户代理';
                    COMMENT ON COLUMN t_user_login_log.device IS '设备';
                    COMMENT ON COLUMN t_user_login_log.system IS '系统';
                    COMMENT ON COLUMN t_user_login_log.browser IS '浏览器';
                    COMMENT ON COLUMN t_user_login_log.desc IS '描述信息';
                    COMMENT ON COLUMN t_user_login_log.login_status IS '登录状态(0:登录成功,1:登录失败,2:已禁用,3:登出)';
                    COMMENT ON COLUMN t_user_login_log.created_at IS '创建时间';
                    COMMENT ON COLUMN t_user_login_log.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_user_login_log` ( -- 用户登录日志表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
                        `user_id` INTEGER NOT NULL, -- 用户ID
                        `username` TEXT NOT NULL, -- 用户名称
                        `token` TEXT DEFAULT '', -- 登录令牌
                        `remote_addr` TEXT DEFAULT '', -- 登录IP
                        `user_agent` TEXT DEFAULT '', -- 用户代理
                        `device` TEXT DEFAULT '', -- 设备
                        `system` TEXT DEFAULT '', -- 系统
                        `browser` TEXT DEFAULT '', -- 浏览器
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `login_status` TINYINT NOT NULL DEFAULT 0, -- 登录状态(0:登录成功,1:登录失败,2:已禁用,3:登出)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_user_login_log_user_id ON t_user_login_log (`user_id`);
                    CREATE INDEX idx_t_user_login_log_username ON t_user_login_log (`username`);
                    CREATE INDEX idx_t_user_login_log_token ON t_user_login_log (`token`);
                    CREATE INDEX idx_t_user_login_log_login_status ON t_user_login_log (`login_status`);
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
            .execute_unprepared("DROP TABLE `t_user_role_rel`")
            .await?;

        Ok(())
    }
}
