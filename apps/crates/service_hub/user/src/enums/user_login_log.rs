/// 用户登录日志表
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 用户登陆状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum LoginStatus {
    /// 登陆成功
    Success = 0,
    /// 登陆失败
    Failed = 1,
    /// 已禁用
    Disabled = 2,
    /// 登出
    Logout = 3,
}
