//! 用户手机号管理

use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use entity::user::user_phone;

/// 查询用户手机号列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetPhonesReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 手机号码
    pub phone: Option<String>,
}

/// 查询用户手机号列表 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetPhonesResp {
    pub data_list: Vec<user_phone::Model>,
    pub total: u64,
}

/// 查询用户手机号信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetPhoneReq {
    /// 手机号ID
    pub id: i32,
}

/// 查询用户手机号信息 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct GetPhoneResp {
    #[serde(flatten)]
    model: user_phone::Model,
}

/// 添加用户手机号 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreatePhoneReq {
    /// 用户ID
    pub user_id: i32,
    /// 手机号码
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,
    /// 描述信息
    pub desc: Option<String>,
}

/// 更新用户手机号 请求体
#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdatePhoneReq {
    /// 手机号ID
    pub id: i32,
    /// 手机号码
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,
    /// 描述信息
    pub desc: Option<String>,
}

// 自定义电话号码验证函数
fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let phone_regex =
        Regex::new(r"^(13[0-9]|14[01456879]|15[0-35-9]|16[2567]|17[0-8]|18[0-9]|19[0-35-9])\d{8}$")
            .map_err(|_err| ValidationError::new("invalid phone"))?;
    if !phone_regex.is_match(phone) {
        return Err(ValidationError::new("invalid phone"));
    }
    Ok(())
}

/// 删除用户手机号 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeletePhoneReq {
    /// 手机号ID
    pub id: i32,
}
