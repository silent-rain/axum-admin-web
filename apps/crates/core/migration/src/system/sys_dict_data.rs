//! 字典数据表
//! Entity: [`entity::system::SysDictData`]

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
                    `t_sys_dict_data` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '字典项ID',
                        `dimension_id` INT(11) NOT NULL COMMENT '字典维度ID',
                        `dimension_code` VARCHAR(64) NOT NULL COMMENT '字典维度编码',
                        `label` VARCHAR(64) NOT NULL COMMENT '字典项标签',
                        `value` TEXT NOT NULL COMMENT '字典项值',
                        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '字典数据表';

                    CREATE INDEX idx_dimension_id ON t_sys_dict_data (`dimension_id`);
                    CREATE INDEX idx_dimension_code ON t_sys_dict_data (`dimension_code`);
                    CREATE INDEX idx_label ON t_sys_dict_data (`label`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_sys_dict_data" (
                        "id" SERIAL PRIMARY KEY,
                        "dimension_id" INTEGER NOT NULL,
                        "dimension_code" VARCHAR(64) NOT NULL,
                        "label" VARCHAR(64) NOT NULL,
                        "value" TEXT NOT NULL,
                        "sort" INTEGER DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" BOOL NOT NULL DEFAULT TRUE,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_sys_dict_data_dimension_id ON t_sys_dict_data ("dimension_id");
                    CREATE INDEX idx_t_sys_dict_data_dimension_code ON t_sys_dict_data ("dimension_code");
                    CREATE INDEX idx_t_sys_dict_data_label ON t_sys_dict_data ("label");

                    COMMENT ON TABLE t_sys_dict_data IS '字典数据表';
                    COMMENT ON COLUMN t_sys_dict_data.id IS '字典项ID';
                    COMMENT ON COLUMN t_sys_dict_data.dimension_id IS '字典维度ID';
                    COMMENT ON COLUMN t_sys_dict_data.dimension_code IS '字典维度编码';
                    COMMENT ON COLUMN t_sys_dict_data.label IS '字典项标签';
                    COMMENT ON COLUMN t_sys_dict_data.value IS '字典项值';
                    COMMENT ON COLUMN t_sys_dict_data.sort IS '排序';
                    COMMENT ON COLUMN t_sys_dict_data.desc IS '描述信息';
                    COMMENT ON COLUMN t_sys_dict_data.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_sys_dict_data.created_at IS '创建时间';
                    COMMENT ON COLUMN t_sys_dict_data.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_sys_dict_data` ( -- 字典数据表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 字典项ID
                        `dimension_id` INTEGER NOT NULL, -- 字典维度ID
                        `dimension_code` TEXT NOT NULL, -- 字典维度编码
                        `label` TEXT NOT NULL, -- 字典项标签
                        `value` TEXT NOT NULL, -- 字典项值
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );
                    
                    CREATE INDEX idx_t_sys_dict_data_dimension_id ON t_sys_dict_data (`dimension_id`);
                    CREATE INDEX idx_t_sys_dict_data_dimension_code ON t_sys_dict_data (`dimension_code`);
                    CREATE INDEX idx_t_sys_dict_data_label ON t_sys_dict_data (`label`);
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
            .execute_unprepared("DROP TABLE `t_sys_dict_data`")
            .await?;

        Ok(())
    }
}
