//! 系统管理
pub mod constant;
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::{
    config::ConfigDao, dict_data::DictDataDao, dict_dimension::DictDimensionDao,
    file_resource::FileResourceDao, image_captcha::ImageCaptchaDao,
};

pub(crate) mod service;
pub use service::{
    config::ConfigService, dict_data::DictDataService, dict_dimension::DictDimensionService,
    file_resource::FileResourceService, image_captcha::ImageCaptchaService,
};

pub(crate) mod controller;
pub use controller::{
    config::ConfigController, dict_data::DictDataController,
    dict_dimension::DictDimensionController, file_resource::FileResourceController,
    image_captcha::ImageCaptchaController,
};

pub(crate) mod router;
pub use router::{
    SystemRouter, config::ConfigRouter, dict_data::DictDataRouter,
    dict_dimension::DictDimensionRouter, file_resource::FileResourceRouter,
    image_captcha::ImageCaptchaRouter,
};
