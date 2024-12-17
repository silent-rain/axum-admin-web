//! 用户手机号表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};

/// 用户手机号表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_user_phone")]
pub struct Model {
    /// 手机号ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 手机号码
    pub phone: String,
    /// 描述信息
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_base::Entity")]
    UserBase,
}

impl Related<super::user_base::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBase.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
