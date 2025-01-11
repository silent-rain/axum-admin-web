//! 部门表
//! Entity: [`entity::organization::Department`]

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
                    `t_org_department` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '部门ID',
                        `pid` BIGINT DEFAULT NULL DEFAULT 0 COMMENT '上级部门ID',
                        `pids` VARCHAR(200) DEFAULT NULL DEFAULT '' COMMENT '所有上级部门ID, 用逗号分开',
                        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '部门名称',
                        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '部门描述',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '部门表';

                    CREATE INDEX idx_pid ON t_org_department (`pid`);
                    CREATE INDEX idx_name ON t_org_department (`name`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_org_department" (
                        "id" SERIAL PRIMARY KEY,
                        "pid" BIGINT DEFAULT 0,
                        "pids" VARCHAR(200) DEFAULT '',
                        "name" VARCHAR(20) UNIQUE NOT NULL,
                        "sort" INTEGER DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_org_department_pid ON t_org_department ("pid");
                    CREATE INDEX idx_t_org_department_name ON t_org_department ("name");

                    COMMENT ON TABLE t_org_department IS '部门表';
                    COMMENT ON COLUMN t_org_department.id IS '部门ID';
                    COMMENT ON COLUMN t_org_department.pid IS '上级部门ID';
                    COMMENT ON COLUMN t_org_department.pids IS '所有上级部门ID, 用逗号分开';
                    COMMENT ON COLUMN t_org_department.name IS '部门名称';
                    COMMENT ON COLUMN t_org_department.sort IS '排序';
                    COMMENT ON COLUMN t_org_department.desc IS '部门描述';
                    COMMENT ON COLUMN t_org_department.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_org_department.created_at IS '创建时间';
                    COMMENT ON COLUMN t_org_department.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_org_department` ( -- 部门表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 部门ID
                        `pid` BIGINT DEFAULT 0, -- 上级部门ID
                        `pids` VARCHAR(200) DEFAULT '', -- 所有上级部门ID, 用逗号分开
                        `name` VARCHAR(20) UNIQUE NOT NULL, -- 部门名称
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` VARCHAR(200) DEFAULT '', -- 部门描述
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_org_department_pid ON t_org_department (`pid`);
                    CREATE INDEX idx_t_org_department_name ON t_org_department (`name`);
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
            .execute_unprepared("DROP TABLE `t_org_department;`")
            .await?;

        Ok(())
    }
}
