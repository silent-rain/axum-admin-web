//! 模板管理

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{Display, EnumString};

/// 定时任务来源
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum Source {
    /// 用户定义
    User = 0,
    /// 系统内部
    System = 1,
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

/// 文本的模式
#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display)]
pub enum TextMode {
    /// 覆盖
    #[strum(to_string = "Overwrite")]
    Overwrite,
    /// 追加
    #[strum(to_string = "Append")]
    Append,
    /// 追加新行
    #[strum(to_string = "Append New Line")]
    AppendNewLine,
}
