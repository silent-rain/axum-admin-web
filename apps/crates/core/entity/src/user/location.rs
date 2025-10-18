//! 用户地理位置表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
    prelude::{DateTime, Decimal, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};

/// 用户地理位置表
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_user_location")]
pub struct Model {
    /// 地理位置ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 省份
    pub province: String,
    /// 城市
    pub city: String,
    /// 区/县
    pub district: String,
    /// 详细地址
    pub address: String,
    /// 邮政编码
    pub postal_code: Option<String>,
    /// 经度
    pub longitude: Option<Decimal>,
    /// 纬度
    pub latitude: Option<Decimal>,
    /// 描述信息
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
    /// 更新时间
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserBase,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserBase => Entity::belongs_to(super::user_base::Entity)
                .from(Column::UserId)
                .to(super::user_base::Column::Id)
                .into(),
        }
    }
}

impl Related<super::user_base::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBase.def()
    }
}

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
