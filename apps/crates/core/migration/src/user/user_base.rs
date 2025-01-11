//! 用户信息表
//! Entity: [`entity::user::UserBase`]

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
                    `t_user_base` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '用户ID',
                        `username` VARCHAR(32) UNIQUE NOT NULL COMMENT '用户名称',
                        `real_name` VARCHAR(32) NULL DEFAULT '' COMMENT '真实姓名',
                        `gender` TINYINT(1) NOT NULL COMMENT '性别(0:男,1:女,2:保密)',
                        `password` VARCHAR(64) NOT NULL COMMENT '密码',
                        `status` BOOL NOT NULL DEFAULT true COMMENT '状态(false:停用,true:正常)',
                        `age` INT(11) NULL DEFAULT 0 COMMENT '年龄',
                        `date_birth` VARCHAR(20) NULL DEFAULT '' COMMENT '出生日期',
                        `avatar` VARCHAR(200) NULL DEFAULT '' COMMENT '头像URL',
                        `intro` VARCHAR(200) NULL DEFAULT '' COMMENT '用户个人介绍',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '用户描述',
                        `address` VARCHAR(200) NULL DEFAULT '' COMMENT '用户的居住或邮寄地址',
                        `share_code` VARCHAR(16) NULL DEFAULT '' COMMENT '用户分享码',
                        `preferences` VARCHAR(200) NULL DEFAULT '' COMMENT '偏好设置',
                        `department_id` INT(11) DEFAULT 0 COMMENT '所属部门ID',
                        `position_id` INT(11) DEFAULT 0 COMMENT '所属岗位ID',
                        `rank_id` INT(11) DEFAULT 0 COMMENT '所属职级ID',
                        `member_level_id` INT(11) DEFAULT 0 COMMENT '用户会员等级ID',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '用户信息表';

                    CREATE INDEX idx_username ON t_user_base (`username`);
                    CREATE INDEX idx_real_name ON t_user_base (`real_name`);
                    CREATE INDEX idx_password ON t_user_base (`password`);
                    CREATE INDEX idx_share_code ON t_user_base (`share_code`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS 
                    "t_user_base" (
                        "id" SERIAL PRIMARY KEY,
                        "username" VARCHAR(32) UNIQUE NOT NULL,
                        "real_name" VARCHAR(32) DEFAULT '',
                        "gender" char NOT NULL,
                        "password" VARCHAR(64) NOT NULL,
                        "status" BOOL NULL DEFAULT TRUE,
                        "age" INT DEFAULT 0,
                        "date_birth" VARCHAR(20) DEFAULT '',
                        "avatar" VARCHAR(200) DEFAULT '',
                        "intro" VARCHAR(200) DEFAULT '',
                        "desc" VARCHAR(200) DEFAULT '',
                        "address" VARCHAR(200) DEFAULT '',
                        "share_code" VARCHAR(16) DEFAULT '',
                        "preferences" VARCHAR(200) DEFAULT '',
                        "department_id" INT DEFAULT 0,
                        "position_id" INT DEFAULT 0,
                        "rank_id" INT DEFAULT 0,
                        "member_level_id" INT DEFAULT 0,
                        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );


                    CREATE INDEX idx_t_user_base_username ON t_user_base ("username");
                    CREATE INDEX idx_t_user_base_real_name ON t_user_base ("real_name");
                    CREATE INDEX idx_t_user_base_password ON t_user_base ("password");
                    CREATE INDEX idx_t_user_base_share_code ON t_user_base ("share_code");

                    COMMENT ON TABLE t_user_base IS '用户信息表';
                    COMMENT ON COLUMN t_user_base.id IS '用户ID';
                    COMMENT ON COLUMN t_user_base.username IS '用户名称';
                    COMMENT ON COLUMN t_user_base.real_name IS '真实姓名';
                    COMMENT ON COLUMN t_user_base.gender IS '性别(0:男,1:女,2:保密)';
                    COMMENT ON COLUMN t_user_base.password IS '密码';
                    COMMENT ON COLUMN t_user_base.status IS '状态(false:停用,true:正常)';
                    COMMENT ON COLUMN t_user_base.age IS '年龄';
                    COMMENT ON COLUMN t_user_base.date_birth IS '出生日期';
                    COMMENT ON COLUMN t_user_base.avatar IS '头像URL';
                    COMMENT ON COLUMN t_user_base.intro IS '用户个人介绍';
                    COMMENT ON COLUMN t_user_base.desc IS '用户描述';
                    COMMENT ON COLUMN t_user_base.address IS '用户的居住或邮寄地址';
                    COMMENT ON COLUMN t_user_base.share_code IS '用户分享码';
                    COMMENT ON COLUMN t_user_base.preferences IS '偏好设置';
                    COMMENT ON COLUMN t_user_base.department_id IS '所属部门ID';
                    COMMENT ON COLUMN t_user_base.position_id IS '所属岗位ID';
                    COMMENT ON COLUMN t_user_base.rank_id IS '所属职级ID';
                    COMMENT ON COLUMN t_user_base.member_level_id IS '用户会员等级ID';
                    COMMENT ON COLUMN t_user_base.created_at IS '创建时间';
                    COMMENT ON COLUMN t_user_base.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_user_base` ( -- 用户信息表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 用户ID,自增
                        `username` VARCHAR(32) NOT NULL UNIQUE, -- 用户名,唯一
                        `real_name` VARCHAR(32) DEFAULT '', -- 真实姓名
                        `gender` TINYINT NOT NULL, -- 性别
                        `password` VARCHAR(64) NOT NULL, -- 密码
                        `status` BOOLEAN NOT NULL DEFAULT true, -- 状态(false:停用,true:正常)
                        `age` INTEGER DEFAULT 0, -- 年龄
                        `date_birth` VARCHAR(20) DEFAULT '', -- 出生日期
                        `avatar` VARCHAR(200) DEFAULT '', -- 头像URL
                        `intro` VARCHAR(200) DEFAULT '', -- 简介
                        `desc` VARCHAR(200) DEFAULT '', -- 描述
                        `address` VARCHAR(200) DEFAULT '', -- 地址
                        `share_code` VARCHAR(16) DEFAULT '', -- 分享码
                        `preferences` VARCHAR(200) DEFAULT '', -- 偏好
                        `department_id` INTEGER DEFAULT 0, -- 部门ID
                        `position_id` INTEGER DEFAULT 0, -- 职位ID
                        `rank_id` INTEGER DEFAULT 0, -- 职级ID
                        `member_level_id` INTEGER DEFAULT 0, -- 会员等级ID
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_user_base_username ON t_user_base (`username`);
                    CREATE INDEX idx_t_user_base_real_name ON t_user_base (`real_name`);
                    CREATE INDEX idx_t_user_base_password ON t_user_base (`password`);
                    CREATE INDEX idx_t_user_base_share_code ON t_user_base (`share_code`);
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
            .execute_unprepared("DROP TABLE `t_user_base`")
            .await?;

        Ok(())
    }
}
