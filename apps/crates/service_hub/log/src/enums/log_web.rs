//! WEB日志表
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 终端类型(0:未知, 1:安卓, 2:IOS, 3:WEB)
#[derive(Debug, Default, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum OsType {
    /// 未知
    #[default]
    Unknown = 0,
    /// 安卓
    Android = 1,
    /// IOS
    IOS = 2,
    /// WEB
    Web = 3,
}

/// 错误类型(0:代码报错, 1:接口报错)
#[derive(Debug, Default, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ErrorType {
    /// 代码报错
    #[default]
    CodeError = 0,
    /// 接口报错
    InterfaceError = 1,
}
