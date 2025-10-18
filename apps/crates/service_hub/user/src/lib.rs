//! 用户信息管理
pub mod cached;
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::{
    blockchain_wallet::BlockchainWalletDao, email::EmailDao, location::LocationDao,
    phone::PhoneDao, role::RoleDao, user_base::UserBaseDao, user_login_log::UserLoginLogDao,
    user_role_rel::UserRoleRelDao, user_session::UserSessionDao,
};

pub(crate) mod service;
pub use service::{
    blockchain_wallet::BlockchainWalletService, email::EmailService, location::LocationService,
    phone::PhoneService, role::RoleService, user_base::UserBaseService,
    user_login_log::UserLoginLogService, user_role_rel::UserRoleRelService,
    user_session::UserSessionService,
};

pub(crate) mod controller;
pub use controller::{
    blockchain_wallet::BlockchainWalletController, email::EmailController,
    location::LocationController, phone::PhoneController, role::RoleController,
    user_base::UserBaseController, user_login_log::UserLoginLogController,
    user_role_rel::UserRoleRelController, user_session::UserSessionController,
};

pub(crate) mod router;
pub use router::{
    UserRouter, blockchain_wallet::BlockchainWalletRouter, email::EmailRouter,
    location::LocationRouter, phone::PhoneRouter, role::RoleRouter, user_base::UserBaseRouter,
    user_login_log::UserLoginLogRouter, user_role_rel::UserRoleRelRouter,
    user_session::UserSessionRouter,
};
