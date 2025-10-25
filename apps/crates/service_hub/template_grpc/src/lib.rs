//! 模板管理
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::t_app_template::AppTemplateDao;

pub(crate) mod service;
pub use service::t_app_template::AppTemplateService;

pub(crate) mod controller;
pub use controller::t_app_template::AppTemplateController;

pub(crate) mod router;
