//! 用户信息表
use std::str::FromStr;

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait, Set,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 用户信息表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_user_base")]
pub struct Model {
    /// 用户ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户名称
    pub username: String,
    /// 真实姓名
    pub real_name: Option<String>,
    /// 性别(0:保密,1:女,2:男)
    pub gender: i8,
    /// 密码
    pub password: String,
    /// 状态(false:停用,true:正常)
    pub status: bool,
    /// 年龄
    pub age: Option<i32>,
    /// 出生日期
    pub date_birth: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 用户个人介绍
    pub intro: Option<String>,
    /// 用户描述
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
    /// 更新时间
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Local::now().naive_local());
        Ok(self)
    }
}

pub mod enums {
    use super::*;

    /// 性别
    #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Gender {
        /// 保密
        Undisclosed = 0,
        /// 女
        Female = 1,
        /// 男
        Male = 2,
    }

    /// 注册用户类型
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum UserType {
        /// 用户名
        #[serde(rename = "base")]
        Base,
        /// 手机号码
        #[serde(rename = "phone")]
        Phone,
        /// 邮箱
        #[serde(rename = "email")]
        Email,
        /// 区块链钱包
        #[serde(rename = "blockchain_wallet")]
        BlockchainWallet,
    }

    impl Default for UserType {
        fn default() -> Self {
            Self::Phone
        }
    }

    /// 实现FromStr trait来定义如何从字符串解析为RegisterType
    impl FromStr for UserType {
        type Err = ();

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match input {
                "phone" => Ok(UserType::Phone),
                "email" => Ok(UserType::Email),
                _ => Err(()),
            }
        }
    }

    impl From<UserType> for String {
        fn from(value: UserType) -> Self {
            match value {
                UserType::Phone => "phone".to_owned(),
                UserType::Email => "email".to_owned(),
                UserType::Base => "base".to_owned(),
                UserType::BlockchainWallet => "blockchain_wallet".to_owned(),
            }
        }
    }
}
