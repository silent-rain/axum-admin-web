//! 图片验证码表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

/// 图片验证码表
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_sys_image_captcha")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 验证码ID
    pub captcha_id: String,
    /// 验证码
    pub captcha: String,
    /// 图片数据, Base64编码
    pub data: Vec<u8>,
    /// 过期时间,秒
    pub expire: u32,
    /// 状态(false:无效验证码,true:有效验证码)
    pub status: bool,
    /// 创建时间
    pub created_at: DateTimeLocal,
    /// 更新时间
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
