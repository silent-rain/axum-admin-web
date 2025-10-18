//! 日志管理
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::{api_operation::ApiOperationDao, system_log::SystemLogDao, web_log::WebLogDao};

pub(crate) mod service;
pub use service::{
    api_operation::ApiOperationService, system_log::SystemLogService, web_log::WebLogService,
};

pub(crate) mod controller;
pub use controller::{
    api_operation::ApiOperationController, system_log::SystemLogController,
    web_log::WebLogController,
};

pub(crate) mod router;
pub use router::{
    LogRouter, api_operation::ApiOperationRouter, system_log::SystemLogRouter,
    web_log::WebLogRouter,
};
