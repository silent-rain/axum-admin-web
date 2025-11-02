//! OpenApi接口表
use serde_repr::{Deserialize_repr, Serialize_repr};

/// OpenApi接口类别
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum Category {
    /// 目录
    Directory = 0,
    /// 接口
    Interface = 1,
}
