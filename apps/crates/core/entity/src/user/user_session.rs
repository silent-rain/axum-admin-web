//! 用户session表

use chrono::Local;
use sea_orm::{
    prelude::{async_trait::async_trait, DateTime},
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait, Set,
};
use serde::{Deserialize, Serialize};

/// 用户session表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_user_session")]
pub struct Model {
    /// session ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 用户名称
    pub username: String,
    /// 用户会话ID
    pub session_id: String,
    /// 描述信息
    pub expiry_date: DateTime,
    /// 元数据
    pub data: Vec<u8>,
    /// 描述信息
    pub desc: Option<String>,
    /// 登录状态是否有效(false:无效,true:有效)
    pub status: bool,
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
