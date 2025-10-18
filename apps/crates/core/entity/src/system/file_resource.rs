//! 文件资源表

use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EnumIter,
    PrimaryKeyTrait, prelude::DateTime,
};
use serde::{Deserialize, Serialize};

/// 文件资源表
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_sys_file_resource")]
pub struct Model {
    /// 文件ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 文件名称
    pub file_name: String,
    /// 文件HASH值
    #[sea_orm(unique)]
    pub hash: String,
    /// 文件数据, Base64编码
    pub data: Vec<u8>,
    /// 文件文件扩展名, 如svg, png
    pub extension: String,
    /// 内容类型, text/html
    /// [content-type](https://www.runoob.com/http/http-content-type.html)
    pub content_type: String,
    /// 文件大小
    pub size: u16,
    /// 描述信息
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub mod enums {
    use super::*;

    /// 文件文件扩展类型, svg,png
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[repr(i8)]
    pub enum ExtensionType {
        #[serde(rename = "svg")]
        Svg,
        #[serde(rename = "png")]
        Png,
    }

    impl From<ExtensionType> for String {
        fn from(value: ExtensionType) -> Self {
            match value {
                ExtensionType::Svg => "svg".to_owned(),
                ExtensionType::Png => "png".to_owned(),
            }
        }
    }
}
