//! 用户信息表

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 性别
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum Gender {
    /// 保密
    Undisclosed = 0,
    /// 女
    Female = 1,
    /// 男
    Male = 2,
}

/// 注册用户类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserType {
    /// 用户名
    #[serde(rename = "base")]
    Base,
    /// 手机号码
    #[serde(rename = "phone")]
    Phone,
    /// 邮箱
    #[serde(rename = "email")]
    Email,
    /// 区块链钱包
    #[serde(rename = "blockchain_wallet")]
    BlockchainWallet,
}

impl Default for UserType {
    fn default() -> Self {
        Self::Phone
    }
}

/// 实现FromStr trait来定义如何从字符串解析为RegisterType
impl FromStr for UserType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "phone" => Ok(UserType::Phone),
            "email" => Ok(UserType::Email),
            _ => Err(()),
        }
    }
}

impl From<UserType> for String {
    fn from(value: UserType) -> Self {
        match value {
            UserType::Phone => "phone".to_owned(),
            UserType::Email => "email".to_owned(),
            UserType::Base => "base".to_owned(),
            UserType::BlockchainWallet => "blockchain_wallet".to_owned(),
        }
    }
}
