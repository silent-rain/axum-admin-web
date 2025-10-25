//! 模板管理
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::t_app_template::TemplateDao;

pub(crate) mod service;
pub use service::t_app_template::TemplateService;

pub(crate) mod controller;
pub use controller::t_app_template::TemplateController;

pub(crate) mod router;
pub use router::TemplateRouter;
