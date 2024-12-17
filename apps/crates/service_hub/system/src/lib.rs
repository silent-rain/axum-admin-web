//! 系统管理
pub mod constant;
pub mod dto;

pub(crate) mod dao;
pub use dao::{
    config::ConfigDao, dict_data::DictDataDao, dict_dimension::DictDimensionDao,
    image_captcha::ImageCaptchaDao, image_resource::ImageResourceDao,
};

pub(crate) mod service;
pub use service::{
    config::ConfigService, dict_data::DictDataService, dict_dimension::DictDimensionService,
    image_captcha::ImageCaptchaService, image_resource::ImageResourceService,
};

pub(crate) mod controller;
pub use controller::{
    config::ConfigController, dict_data::DictDataController,
    dict_dimension::DictDimensionController, image_captcha::ImageCaptchaController,
    image_resource::ImageResourceController,
};

pub(crate) mod router;
pub use router::{
    config::ConfigRouter, dict_data::DictDataRouter, dict_dimension::DictDimensionRouter,
    image_captcha::ImageCaptchaRouter, image_resource::ImageResourceRouter, SystemRouter,
};
