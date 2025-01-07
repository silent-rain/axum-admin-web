//! 路由层
pub mod config;
pub mod dict_data;
pub mod dict_dimension;
pub mod file_resource;
pub mod image_captcha;

use axum::Router;

/// 路由器
pub struct SystemRouter;

impl SystemRouter {
    /// 注册`系统管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/system",
            Router::new()
                .merge(image_captcha::ImageCaptchaRouter::register()) // 图片验证码管理
                .merge(file_resource::FileResourceRouter::register()) // 图片资源管理
                .merge(config::ConfigRouter::register()) // 配置管理
                .merge(dict_dimension::DictDimensionRouter::register()) // 字典维度管理
                .merge(dict_data::DictDataRouter::register()), // 字典数据管理
        )
    }
}
