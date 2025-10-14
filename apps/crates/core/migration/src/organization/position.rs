//! 岗位表
//! Entity: [`entity::organization::Position`]

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
                    `t_org_position` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '岗位ID',
                        `name` VARCHAR(100) UNIQUE NOT NULL COMMENT '岗位名称',
                        `sort` INT NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '岗位描述',
                        `department_id` INT DEFAULT 0 COMMENT '所属部门ID',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '岗位表';

                    CREATE INDEX idx_name ON t_org_position (`name`);
                    CREATE INDEX idx_department_id ON t_org_position (`department_id`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_org_position" (
                        "id" SERIAL PRIMARY KEY,
                        "name" VARCHAR(100) UNIQUE NOT NULL,
                        "sort" INTEGER DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "department_id" INTEGER DEFAULT 0,
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_org_position_name ON t_org_position ("name");
                    CREATE INDEX idx_t_org_position_department_id ON t_org_position ("department_id");

                    COMMENT ON TABLE t_org_position IS '岗位表';
                    COMMENT ON COLUMN t_org_position.id IS '岗位ID';
                    COMMENT ON COLUMN t_org_position.name IS '岗位名称';
                    COMMENT ON COLUMN t_org_position.sort IS '排序';
                    COMMENT ON COLUMN t_org_position.desc IS '岗位描述';
                    COMMENT ON COLUMN t_org_position.department_id IS '所属部门ID';
                    COMMENT ON COLUMN t_org_position.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_org_position.created_at IS '创建时间';
                    COMMENT ON COLUMN t_org_position.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_org_position` ( -- 岗位表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 岗位ID
                        `name` VARCHAR(100) UNIQUE NOT NULL, -- 岗位名称
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` VARCHAR(200) DEFAULT '', -- 岗位描述
                        `department_id` INTEGER DEFAULT 0, -- 所属部门ID
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_org_position_name ON t_org_position (`name`);
                    CREATE INDEX idx_t_org_position_department_id ON t_org_position (`department_id`);
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
            .execute_unprepared("DROP TABLE `t_org_position;`")
            .await?;

        Ok(())
    }
}
