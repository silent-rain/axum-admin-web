//! 文件资源表
use serde::{Deserialize, Serialize};

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
