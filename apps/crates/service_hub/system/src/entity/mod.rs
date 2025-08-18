//! 系统相关表

pub mod config;
pub mod dict_data;
pub mod dict_dimension;
pub mod file_resource;
pub mod image_captcha;

pub use config::Entity as ConfigEntity;
pub use dict_data::Entity as DictDataEntity;
pub use dict_dimension::Entity as DictDimensionEntity;
pub use file_resource::Entity as FileResourceEntity;
pub use image_captcha::Entity as ImageCaptchaEntity;
