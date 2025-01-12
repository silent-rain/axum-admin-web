//! OpenApi接口表
//! Entity: [`entity::permission::Openapi`]

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
                    t_perm_openapi (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '接口ID',
                        `pid` INT NULL DEFAULT 0 COMMENT '父ID',
                        `category` TINYINT(1) NOT NULL COMMENT '类别,0:目录,1:接口',
                        `name` VARCHAR(50) NOT NULL COMMENT '接口名称',
                        `method` VARCHAR(50) NOT NULL COMMENT '请求类型',
                        `path` VARCHAR(200) NOT NULL COMMENT '资源路径',
                        `sort` INT NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT 'OpenApi接口表';

                    CREATE INDEX idx_pid ON t_perm_openapi (`pid`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_perm_openapi" (
                        "id" SERIAL PRIMARY KEY,
                        "pid" INTEGER DEFAULT 0,
                        "category" char NOT NULL,
                        "name" VARCHAR(50) NOT NULL,
                        "method" VARCHAR(50) NOT NULL,
                        "path" VARCHAR(200) NOT NULL,
                        "sort" INTEGER DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_perm_openapi_pid ON t_perm_openapi ("pid");

                    COMMENT ON TABLE t_perm_openapi IS 'OpenApi接口表';
                    COMMENT ON COLUMN t_perm_openapi.id IS '接口ID';
                    COMMENT ON COLUMN t_perm_openapi.pid IS '父ID';
                    COMMENT ON COLUMN t_perm_openapi.category IS '类别,0:目录,1:接口';
                    COMMENT ON COLUMN t_perm_openapi.name IS '接口名称';
                    COMMENT ON COLUMN t_perm_openapi.method IS '请求类型';
                    COMMENT ON COLUMN t_perm_openapi.path IS '资源路径';
                    COMMENT ON COLUMN t_perm_openapi.sort IS '排序';
                    COMMENT ON COLUMN t_perm_openapi.desc IS '描述信息';
                    COMMENT ON COLUMN t_perm_openapi.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_perm_openapi.created_at IS '创建时间';
                    COMMENT ON COLUMN t_perm_openapi.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_perm_openapi` ( -- OpenApi接口表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 接口ID
                        `pid` INTEGER DEFAULT 0, -- 父ID
                        `category` TINYINT NOT NULL, -- 类别,0:目录,1:接口
                        `name` VARCHAR(50) NOT NULL, -- 接口名称
                        `method` VARCHAR(50) NOT NULL, -- 请求类型
                        `path` VARCHAR(200) NOT NULL, -- 资源路径
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` VARCHAR(200) DEFAULT '', -- 描述信息
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_perm_openapi_pid ON t_perm_openapi (`pid`);
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
            .execute_unprepared("DROP TABLE `t_perm_openapi`")
            .await?;

        Ok(())
    }
}
