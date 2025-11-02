//! 系统相关表

pub mod config;
pub mod dict_data;
pub mod dict_dimension;
pub mod file_resource;
pub mod image_captcha;

pub use config::Entity as Config;
pub use dict_data::Entity as DictData;
pub use dict_dimension::Entity as DictDimension;
pub use file_resource::Entity as FileResource;
pub use image_captcha::Entity as ImageCaptcha;
