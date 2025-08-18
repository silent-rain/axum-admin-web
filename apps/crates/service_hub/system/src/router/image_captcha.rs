//! 图片验证码管理

use axum::{
    Router,
    routing::{delete, get},
};

use crate::controller::image_captcha::ImageCaptchaController;

/// 路由器
pub struct ImageCaptchaRouter;

impl ImageCaptchaRouter {
    /// 注册`图片验证码管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/image-captchas",
            Router::new()
                .route(
                    "/",
                    get(ImageCaptchaController::list).post(ImageCaptchaController::create),
                )
                .route(
                    "/{id}",
                    get(ImageCaptchaController::info).delete(ImageCaptchaController::delete),
                )
                .route(
                    "/batch_delete",
                    delete(ImageCaptchaController::batch_delete),
                )
                .route("/show_image", get(ImageCaptchaController::show_image)),
        )
    }
}
