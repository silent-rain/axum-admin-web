//! 登陆

use serde::{Deserialize, Serialize};
use validator::Validate;

use user::enums::user_base::UserType;

/// 登陆 请求体
#[derive(Default, Clone, Deserialize, Validate)]
pub struct LoginReq {
    /// 注册用户类型
    pub user_type: UserType,
    /// 用户名
    pub username: Option<String>,
    /// 手机号码
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 区块链钱包
    pub blockchain_wallet: Option<String>,

    /// 登陆密码
    pub password: String,
    /// 验证码ID
    pub captcha_id: String,
    /// 验证码
    pub captcha: String,
}

/// 登陆 响应体
#[derive(Default, Deserialize, Serialize)]
pub struct LoginResp {
    /// 用户ID
    pub user_id: i32,
}

/// 浏览器信息
#[derive(Default, Deserialize, Serialize)]
pub struct BrowserInfo {
    /// Peer socket address.
    pub remote_addr: String,
    /// User Agent
    pub user_agent: String,
}
