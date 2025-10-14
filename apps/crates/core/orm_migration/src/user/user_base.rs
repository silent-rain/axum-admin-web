//! 用户信息表
//! Entity: [`entity::user::UserBase`]

use sea_orm::{
    DeriveIden, DeriveMigrationName,
    sea_query::{ColumnDef, Expr, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

use crate::utils::if_not_exists_create_index;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(UserBase::Table)
                    .comment("用户信息表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserBase::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Username)
                            .string()
                            .string_len(32)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(UserBase::RealName)
                            .string()
                            .string_len(32)
                            .null()
                            .default("")
                            .comment("真实姓名"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Gender)
                            .tiny_integer()
                            .not_null()
                            .default(0)
                            .comment("性别(0:保密,1:女,2:男)"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Password)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("密码"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Status)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Age)
                            .integer()
                            .null()
                            .comment("年龄"),
                    )
                    .col(
                        ColumnDef::new(UserBase::DateBirth)
                            .string()
                            .string_len(20)
                            .null()
                            .default("")
                            .comment("出生日期"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Avatar)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("头像URL"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Intro)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("用户个人介绍"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("用户描述"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Address)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("用户的居住或邮寄地址"),
                    )
                    .col(
                        ColumnDef::new(UserBase::ShareCode)
                            .string()
                            .string_len(16)
                            .null()
                            .default("")
                            .comment("用户分享码"),
                    )
                    .col(
                        ColumnDef::new(UserBase::Preferences)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("偏好设置"),
                    )
                    .col(
                        ColumnDef::new(UserBase::DepartmentId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属部门ID"),
                    )
                    .col(
                        ColumnDef::new(UserBase::PositionId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属岗位ID"),
                    )
                    .col(
                        ColumnDef::new(UserBase::RankId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("所属职级ID"),
                    )
                    .col(
                        ColumnDef::new(UserBase::MemberLevelId)
                            .integer()
                            .null()
                            .default(0)
                            .comment("用户会员等级ID"),
                    )
                    .col(
                        ColumnDef::new(UserBase::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserBase::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if_not_exists_create_index(manager, UserBase::Table, vec![UserBase::Username]).await?;
        if_not_exists_create_index(manager, UserBase::Table, vec![UserBase::RealName]).await?;
        if_not_exists_create_index(manager, UserBase::Table, vec![UserBase::Password]).await?;
        if_not_exists_create_index(manager, UserBase::Table, vec![UserBase::ShareCode]).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserBase::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserBase {
    #[sea_orm(iden = "t_user_base")]
    Table,
    Id,
    Username,
    RealName,
    Gender,
    Password,
    Status,
    Age,
    DateBirth,
    Avatar,
    Intro,
    Desc,
    Address,
    ShareCode,
    Preferences,
    DepartmentId,
    PositionId,
    RankId,
    MemberLevelId,
    CreatedAt,
    UpdatedAt,
}
