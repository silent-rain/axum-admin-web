//! 令牌表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait, Set,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};

/// 令牌表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_token")]
pub struct Model {
    /// 令牌ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 令牌
    pub token: String,
    /// 口令
    pub passphrase: String,
    /// 权限范围:GET,POST,PUT,DELETE
    pub permission: String,
    /// 授权到期时间
    pub expire: DateTime,
    /// 状态(false:停用,true:正常)
    pub status: bool,
    /// 描述信息
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

    /// 令牌权限范围
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum Permission {
        /// 读取数据
        #[serde(rename = "GET")]
        GET,
        /// 提交数据
        #[serde(rename = "POST")]
        POST,
        /// 更新数据
        #[serde(rename = "PUT")]
        PUT,
        /// 删除数据
        #[serde(rename = "DELETE")]
        DELETE,
    }
}
