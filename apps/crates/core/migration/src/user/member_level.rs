//! 用户会员等级表
//! Entity: [`entity::user::MemberLevel`]

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
                    CREATE TABLE
                    `t_user_member_level` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '会员等级ID',
                        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '会员等级名称',
                        `level` INT(11) UNSIGNED UNIQUE NOT NULL COMMENT '会员等级',
                        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '会员描述',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '用户会员等级表';

                    CREATE INDEX idx_name ON t_user_member_level (`name`);
                    CREATE INDEX idx_level ON t_user_member_level (`level`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS 
                    "t_user_member_level" (
                        "id" SERIAL PRIMARY KEY,
                        "name" VARCHAR(20) UNIQUE NOT NULL,
                        "level" INT UNIQUE NOT NULL,
                        "sort" INT DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_user_member_level_name ON t_user_member_level ("name");
                    CREATE INDEX idx_t_user_member_level_level ON t_user_member_level ("level");

                    COMMENT ON TABLE t_user_member_level IS '用户会员等级表';
                    COMMENT ON COLUMN t_user_member_level.id IS '会员等级ID';
                    COMMENT ON COLUMN t_user_member_level.name IS '会员等级名称';
                    COMMENT ON COLUMN t_user_member_level.level IS '会员等级';
                    COMMENT ON COLUMN t_user_member_level.sort IS '排序';
                    COMMENT ON COLUMN t_user_member_level.desc IS '会员描述';
                    COMMENT ON COLUMN t_user_member_level.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_user_member_level.created_at IS '创建时间';
                    COMMENT ON COLUMN t_user_member_level.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    t_user_member_level ( -- 用户会员等级表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 会员等级ID
                        `name` TEXT UNIQUE NOT NULL, -- 会员等级名称
                        `level` INTEGER UNIQUE NOT NULL, -- 会员等级
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` TEXT DEFAULT '', -- 会员描述
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_user_member_level_name ON t_user_member_level (`name`);
                    CREATE INDEX idx_t_user_member_level_level ON t_user_member_level (`level`);
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
            .execute_unprepared("DROP TABLE `t_user_member_level`")
            .await?;

        Ok(())
    }
}
