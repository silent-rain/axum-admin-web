//! 模板管理
pub mod dto;
pub mod entity;
pub mod enums;

pub(crate) mod dao;
pub use dao::template::TemplateDao;

pub(crate) mod service;
pub use service::template::TemplateService;

pub(crate) mod controller;
pub use controller::template::TemplateController;

pub(crate) mod router;
pub use router::TemplateRouter;
