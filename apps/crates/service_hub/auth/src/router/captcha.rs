//! 验证码

use axum::{routing::get, Router};
use system::ImageCaptchaController;

/// 路由器
pub struct GenCaptchaRouter;

impl GenCaptchaRouter {
    /// 注册`生成验证码`路由
    pub fn register() -> Router {
        Router::new().route("/captcha", get(ImageCaptchaController::create))
    }
}
