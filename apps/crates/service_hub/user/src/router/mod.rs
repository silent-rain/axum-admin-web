//! 路由层

pub mod blockchain_wallet;
pub mod email;
pub mod location;
pub mod member_level;
pub mod phone;
pub mod role;
pub mod user_base;
pub mod user_role_rel;

use actix_web::{web, Scope};

/// 路由器
pub struct UserRouter;

impl UserRouter {
    /// 注册`用户管理`路由
    pub fn admin_register() -> Scope {
        web::scope("/user")
            // 角色管理
            .service(role::RoleRouter::admin_register())
            // 用户信息管理
            .service(user_base::UserBaseRouter::admin_register())
            // 用户手机号管理
            .service(phone::PhoneRouter::admin_register())
            // 用户邮箱管理
            .service(email::EmailRouter::admin_register())
            // 用户区块链钱包管理
            .service(blockchain_wallet::BlockchainWalletRouter::admin_register())
            // 会员等级管理
            .service(member_level::MemberLevelRouter::admin_register())
            // 用户地理位置管理
            .service(location::LocationRouter::admin_register())
    }
}
