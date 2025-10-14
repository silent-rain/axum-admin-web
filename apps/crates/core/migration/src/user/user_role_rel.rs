//! 用户角色关系表
//! Entity: [`entity::user::UserRoleRel`]

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
                    `t_user_role_rel` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '自增ID',
                        `user_id` INT NOT NULL COMMENT '用户ID',
                        `role_id` INT NOT NULL COMMENT '角色ID',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '用户角色关系表';

                    CREATE UNIQUE INDEX uk_user_id_role_id ON t_user_role_rel (`user_id`, `role_id`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS
                    "t_user_role_rel" (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" INT NOT NULL,
                        "role_id" INT NOT NULL,
                        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE UNIQUE INDEX uk_t_user_role_rel_user_id_role_id ON t_user_role_rel ("user_id", "role_id");

                    COMMENT ON TABLE t_user_role_rel IS '用户角色关系表';
                    COMMENT ON COLUMN t_user_role_rel.id IS '自增ID';
                    COMMENT ON COLUMN t_user_role_rel.user_id IS '用户ID';
                    COMMENT ON COLUMN t_user_role_rel.role_id IS '角色ID';
                    COMMENT ON COLUMN t_user_role_rel.created_at IS '创建时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_user_role_rel` ( -- 用户角色关系表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
                        `user_id` INTEGER NOT NULL, -- 用户ID
                        `role_id` INTEGER NOT NULL, -- 角色ID
                        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 创建时间
                    );

                    CREATE UNIQUE INDEX uk_t_user_role_rel_user_id_role_id ON t_user_role_rel (`user_id`, `role_id`);
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
