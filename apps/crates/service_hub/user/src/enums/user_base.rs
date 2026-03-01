//! 用户信息表

use serde_repr::{Deserialize_repr, Serialize_repr};

/// 性别
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum Gender {
    /// 保密
    Undisclosed = 0,
    /// 女
    Female = 1,
    /// 男
    Male = 2,
}

/*
use std::str::FromStr;

/// 注册用户类型
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserType {
    /// 用户名
    #[serde(rename = "base")]
    Base,
    /// 手机号码
    #[default]
    #[serde(rename = "phone")]
    Phone,
    /// 邮箱
    #[serde(rename = "email")]
    Email,
    /// 区块链钱包
    #[serde(rename = "blockchain_wallet")]
    UserBlockchainWallet,
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
            UserType::UserBlockchainWallet => "blockchain_wallet".to_owned(),
        }
    }
}

*/

/// 注册用户类型
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum UserType {
    /// 用户名
    Base = 0,
    /// 手机号码
    #[default]
    Phone = 1,
    /// 邮箱
    Email = 2,
    /// 区块链钱包
    UserBlockchainWallet,
}
