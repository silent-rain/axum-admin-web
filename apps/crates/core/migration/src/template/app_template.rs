//! 应用模板表
//! Entity: [`entity::template::AppTemplate`]

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
                    `t_app_template` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '模板ID',
                        `user_id` INT(11) NOT NULL COMMENT '用户ID',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '登录状态(0:登陆成功,1:登陆失败,2:已禁用,3:登出)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '应用模板表';

                    CREATE INDEX idx_user_id ON t_app_template (`user_id`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS t_app_template (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" INTEGER NOT NULL,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" SMALLINT NOT NULL DEFAULT 0,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_app_template_user_id ON t_app_template ("user_id");

                    COMMENT ON TABLE t_app_template IS '应用模板表';
                    COMMENT ON COLUMN t_app_template.id IS '模板ID';
                    COMMENT ON COLUMN t_app_template.user_id IS '用户ID';
                    COMMENT ON COLUMN t_app_template.desc IS '描述信息';
                    COMMENT ON COLUMN t_app_template.status IS '登录状态(0:登陆成功,1:登陆失败,2:已禁用,3:登出)';
                    COMMENT ON COLUMN t_app_template.created_at IS '创建时间';
                    COMMENT ON COLUMN t_app_template.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_app_template` ( -- 应用模板表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 模板ID
                        `user_id` INTEGER NOT NULL, -- 用户ID
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `status` INTEGER NOT NULL DEFAULT 0, -- 登录状态(0:登陆成功,1:登陆失败,2:已禁用,3:登出)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_app_template_user_id ON t_app_template (`user_id`);
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
            .execute_unprepared("DROP TABLE `t_app_template`")
            .await?;

        Ok(())
    }
}
