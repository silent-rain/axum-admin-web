//! 菜单表

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 菜单类型
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum MenuType {
    /// 菜单
    Menu = 0,
    /// 按钮
    Button = 1,
}

/// 菜单打开方式
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum OpenMethod {
    /// 组件
    Component = 0,
    /// 内链
    InternalLink = 1,
    /// 外链
    ExternalLink = 2,
}

/// 菜单链接跳转方式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LinkTarget {
    /// 新窗口中打开
    #[serde(rename = "_blank")]
    Blank,
    /// 当前窗口中打开
    #[serde(rename = "_self")]
    Current,
}

impl From<LinkTarget> for String {
    fn from(value: LinkTarget) -> Self {
        match value {
            LinkTarget::Blank => "_blank".to_owned(),
            LinkTarget::Current => "_self".to_owned(),
        }
    }
}
