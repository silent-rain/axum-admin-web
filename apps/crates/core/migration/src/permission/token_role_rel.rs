//! 令牌角色关系表
//! Entity: [`entity::permission::TokenRoleRel`]

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
                    `t_perm_token_role_rel` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '自增ID',
                        `token_id` INT(11) NOT NULL COMMENT '令牌ID',
                        `role_id` INT(11) NOT NULL COMMENT '角色ID',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '令牌角色关系表';

                    CREATE UNIQUE INDEX uk_token_id_role_id ON t_perm_token_role_rel (`token_id`, `role_id`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_perm_token_role_rel" (
                        "id" SERIAL PRIMARY KEY,
                        "token_id" INTEGER NOT NULL,
                        "role_id" INTEGER NOT NULL,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE UNIQUE INDEX uk_t_perm_token_role_rel_token_id_role_id ON t_perm_token_role_rel ("token_id", "role_id");

                    COMMENT ON TABLE t_perm_token_role_rel IS '令牌角色关系表';
                    COMMENT ON COLUMN t_perm_token_role_rel.id IS '自增ID';
                    COMMENT ON COLUMN t_perm_token_role_rel.token_id IS '令牌ID';
                    COMMENT ON COLUMN t_perm_token_role_rel.role_id IS '角色ID';
                    COMMENT ON COLUMN t_perm_token_role_rel.created_at IS '创建时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_perm_token_role_rel` ( -- 令牌角色关系表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
                        `token_id` INTEGER NOT NULL, -- 令牌ID
                        `role_id` INTEGER NOT NULL, -- 角色ID
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 创建时间
                    );

                    CREATE UNIQUE INDEX uk_t_perm_token_role_rel_token_id_role_id ON t_perm_token_role_rel (`token_id`, `role_id`);
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
            .execute_unprepared("DROP TABLE `t_perm_token_role_rel`")
            .await?;

        Ok(())
    }
}
