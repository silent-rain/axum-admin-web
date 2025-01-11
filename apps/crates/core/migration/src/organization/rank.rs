//! 职级表
//! Entity: [`entity::organization::Rank`]

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
                    `t_org_rank` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '职级ID',
                        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '职级名称',
                        `level` INT UNSIGNED UNIQUE NOT NULL COMMENT '职级等级',
                        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '职级描述',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '职级表';

                    CREATE INDEX idx_name ON t_org_rank (`name`);
                    CREATE INDEX idx_level ON t_org_rank (`level`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_org_rank" (
                        "id" SERIAL PRIMARY KEY,
                        "name" VARCHAR(20) UNIQUE NOT NULL,
                        "level" INTEGER UNIQUE NOT NULL,
                        "sort" INTEGER DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_org_rank_name ON t_org_rank ("name");
                    CREATE INDEX idx_t_org_rank_level ON t_org_rank ("level");

                    COMMENT ON TABLE t_org_rank IS '职级表';
                    COMMENT ON COLUMN t_org_rank.id IS '职级ID';
                    COMMENT ON COLUMN t_org_rank.name IS '职级名称';
                    COMMENT ON COLUMN t_org_rank.level IS '职级等级';
                    COMMENT ON COLUMN t_org_rank.sort IS '排序';
                    COMMENT ON COLUMN t_org_rank.desc IS '职级描述';
                    COMMENT ON COLUMN t_org_rank.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_org_rank.created_at IS '创建时间';
                    COMMENT ON COLUMN t_org_rank.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_org_rank` ( -- 职级表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 职级ID
                        `name` VARCHAR(20) UNIQUE NOT NULL, -- 职级名称
                        `level` INTEGER UNIQUE NOT NULL, -- 职级等级
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` VARCHAR(200) DEFAULT '', -- 职级描述
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_org_rank_name ON t_org_rank (`name`);
                    CREATE INDEX idx_t_org_rank_level ON t_org_rank (`level`);
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
            .execute_unprepared("DROP TABLE `t_org_rank;`")
            .await?;

        Ok(())
    }
}
