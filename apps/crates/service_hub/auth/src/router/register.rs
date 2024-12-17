//! 注册用户路由

use crate::controller::register::RegisterController;

use actix_web::{web, Scope};

/// 路由器
pub struct RegisterRouter;

impl RegisterRouter {
    /// 注册`注册用户`路由
    pub fn register() -> Scope {
        web::scope("/register").route("", web::post().to(RegisterController::register))
    }
}
