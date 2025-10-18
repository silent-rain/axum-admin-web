//! 用户登录日志表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait, Set,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 用户登录日志表
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_user_login_log")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 用户名称
    pub username: String,
    /// 用户会话ID
    pub session_id: String,
    /// 登录IP
    pub remote_addr: String,
    /// 用户代理
    pub user_agent: String,
    /// 设备
    pub device: Option<String>,
    /// 系统
    pub system: Option<String>,
    /// 浏览器
    pub browser: Option<String>,
    /// 描述信息
    pub desc: Option<String>,
    /// 登录状态
    pub login_status: i8,
    /// 创建时间
    pub created_at: DateTime,
    /// 更新时间
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
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

    /// 用户登陆状态
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum LoginStatus {
        /// 登陆成功
        Success = 0,
        /// 登陆失败
        Failed = 1,
        /// 已禁用
        Disabled = 2,
        /// 登出
        Logout = 3,
    }
}
