//! 路由层

pub mod blockchain_wallet;
pub mod email;
pub mod location;
pub mod member_level;
pub mod phone;
pub mod role;
pub mod user_base;
pub mod user_login_log;
pub mod user_role_rel;
pub mod user_session;

use axum::Router;

/// 路由器
pub struct UserRouter;

impl UserRouter {
    /// 注册`用户管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/user",
            Router::new()
                .merge(role::RoleRouter::register()) // 角色管理
                .merge(user_base::UserBaseRouter::register()) // 用户信息管理
                .merge(phone::PhoneRouter::register()) // 用户手机号管理
                .merge(email::EmailRouter::register()) // 用户邮箱管理
                .merge(blockchain_wallet::BlockchainWalletRouter::register()) // 用户区块链钱包管理
                .merge(member_level::MemberLevelRouter::register()) // 会员等级管理
                .merge(location::LocationRouter::register()) // 用户地理位置管理
                .merge(user_login_log::UserLoginLogRouter::register()), // 登陆日志管理
        )
    }
}
