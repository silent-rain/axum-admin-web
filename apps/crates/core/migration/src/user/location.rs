//! 用户地理位置表
//! Entity: [`entity::user::Location`]

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
                    `t_user_location` (
                        `id` INT AUTO_INCREMENT NOT NULL COMMENT '地理位置ID',
                        `user_id` INT(10) UNIQUE NOT NULL COMMENT '用户ID',
                        `province` VARCHAR(50) NOT NULL COMMENT '省份',
                        `city` VARCHAR(50) NOT NULL COMMENT '城市',
                        `district` VARCHAR(50) NOT NULL COMMENT '区/县',
                        `address` VARCHAR(255) NOT NULL COMMENT '详细地址',
                        `postal_code` VARCHAR(20) NULL DEFAULT '' COMMENT '邮政编码',
                        `longitude` DECIMAL(11, 8) NULL DEFAULT 0 COMMENT '经度',
                        `latitude` DECIMAL(10, 8) NULL DEFAULT 0 COMMENT '纬度',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '用户地理位置表';
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS 
                    "t_user_location" (
                        "id" SERIAL PRIMARY KEY, 
                        "user_id" INT UNIQUE NOT NULL, 
                        "province" VARCHAR(50) NOT NULL, 
                        "city" VARCHAR(50) NOT NULL, 
                        "district" VARCHAR(50) NOT NULL, 
                        "address" VARCHAR(255) NOT NULL, 
                        "postal_code" VARCHAR(20) DEFAULT '', 
                        "longitude" DECIMAL(11, 8) DEFAULT 0, 
                        "latitude" DECIMAL(10, 8) DEFAULT 0, 
                        "desc" VARCHAR(200) DEFAULT '', 
                        "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP, 
                        "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP 
                    );

                    COMMENT ON TABLE t_user_location IS '用户地理位置表';
                    COMMENT ON COLUMN t_user_location.id IS '地理位置ID';
                    COMMENT ON COLUMN t_user_location.user_id IS '用户ID';
                    COMMENT ON COLUMN t_user_location.province IS '省份';
                    COMMENT ON COLUMN t_user_location.city IS '城市';
                    COMMENT ON COLUMN t_user_location.district IS '区/县';
                    COMMENT ON COLUMN t_user_location.address IS '详细地址';
                    COMMENT ON COLUMN t_user_location.postal_code IS '邮政编码';
                    COMMENT ON COLUMN t_user_location.longitude IS '经度';
                    COMMENT ON COLUMN t_user_location.latitude IS '纬度';
                    COMMENT ON COLUMN t_user_location.desc IS '描述信息';
                    COMMENT ON COLUMN t_user_location.created_at IS '创建时间';
                    COMMENT ON COLUMN t_user_location.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    t_user_location (  -- 用户地理位置表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 地理位置ID
                        `user_id` INTEGER UNIQUE NOT NULL, -- 用户ID
                        `province` TEXT NOT NULL, -- 省份
                        `city` TEXT NOT NULL, -- 城市
                        `district` TEXT NOT NULL, -- 区/县
                        `address` TEXT NOT NULL, -- 详细地址
                        `postal_code` TEXT DEFAULT '', -- 邮政编码
                        `longitude` REAL DEFAULT 0, -- 经度
                        `latitude` REAL DEFAULT 0, -- 纬度
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );
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
            .execute_unprepared("DROP TABLE `t_user_location`")
            .await?;

        Ok(())
    }
}
