//! 应用模板, 用于制作自定义服务模板

use chrono::Local;
use sea_orm::{
    prelude::{async_trait::async_trait, DateTime},
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Set,
};
use serde::{Deserialize, Serialize};

/// 应用模板
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_app_template")]
pub struct Model {
    /// 模板ID
    #[serde(default)]
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
    pub status: i8,
    /// 创建时间
    pub created_at: DateTime,
    /// 更新时间
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::user::user_base::Entity",
        from = "Column::UserId",
        to = "crate::user::user_base::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    UserBase,
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
/// 枚举
pub mod enums {
    use serde_repr::{Deserialize_repr, Serialize_repr};

    /// 状态
    #[derive(Debug, Default, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum Status {
        /// 停用
        #[default]
        Disabled = 0,
        /// 正常
        Enabled = 1,
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use sea_orm::prelude::DateTimeLocal;

    #[test]
    fn it_works() {
        let result = DateTimeLocal::default();
        println!("result default: {}", result.to_string());
        println!("result default: {}", result.to_rfc3339());

        let result = Local::now();
        println!("result now: {}", result.to_string());
        println!("result now: {}", result.to_rfc3339());

        let result = Local::now().naive_local();
        println!("result naive_local: {}", result.to_string());
    }
}
