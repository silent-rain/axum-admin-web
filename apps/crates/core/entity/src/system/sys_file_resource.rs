//! 文件资源表

use sea_orm::{
    prelude::DateTimeLocal, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait,
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
    pub name: String,
    /// 文件HASH值
    #[sea_orm(unique)]
    pub hash: String,
    /// 文件数据, Base64编码
    pub data: Vec<u8>,
    /// 文件文件扩展名, 如svg, png
    pub extension: String,
    /// 文件大小
    pub size: u16,
    /// 描述信息
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 枚举
pub mod enums {
    use serde::{Deserialize, Serialize};

    /// ICON文件扩展类型,svg,png
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[repr(i8)]
    pub enum ImageType {
        /// 无效验证码
        #[serde(rename = "svg")]
        Svg,
        /// 有效验证码
        #[serde(rename = "png")]
        Png,
    }

    impl From<ImageType> for String {
        fn from(value: ImageType) -> Self {
            match value {
                ImageType::Svg => "svg".to_owned(),
                ImageType::Png => "png".to_owned(),
            }
        }
    }
}
