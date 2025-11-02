//! 用户信息管理
pub mod cached;
pub(crate) mod controller;
pub(crate) mod dao;
pub mod dto;
pub mod enums;
pub(crate) mod router;
pub(crate) mod service;

pub use dao::{
    location::LocationDao, role::RoleDao, user_base::UserBaseDao,
    user_blockchain_wallet::BlockchainWalletDao, user_email::EmailDao,
    user_login_log::UserLoginLogDao, user_phone::PhoneDao, user_role_rel::UserRoleRelDao,
    user_session::UserSessionDao,
};

pub use service::{
    location::LocationService, role::RoleService, user_base::UserBaseService,
    user_blockchain_wallet::BlockchainWalletService, user_email::EmailService,
    user_login_log::UserLoginLogService, user_phone::PhoneService,
    user_role_rel::UserRoleRelService, user_session::UserSessionService,
};

pub use controller::{
    location::LocationController, role::RoleController, user_base::UserBaseController,
    user_blockchain_wallet::BlockchainWalletController, user_email::EmailController,
    user_login_log::UserLoginLogController, user_phone::PhoneController,
    user_role_rel::UserRoleRelController, user_session::UserSessionController,
};

pub use router::{
    UserRouter, location::LocationRouter, role::RoleRouter, user_base::UserBaseRouter,
    user_blockchain_wallet::BlockchainWalletRouter, user_email::EmailRouter,
    user_login_log::UserLoginLogRouter, user_phone::PhoneRouter, user_role_rel::UserRoleRelRouter,
    user_session::UserSessionRouter,
};
