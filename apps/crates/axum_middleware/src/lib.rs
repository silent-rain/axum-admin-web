//! 中间件
pub mod constant;
pub mod error;

pub mod api_operation_log;
pub mod api_operation_log_fn;
pub mod empty_wrapper_fn;

// 鉴权
pub mod casbin_auth;
pub mod check_auth;
pub mod jwt_auth;
pub mod openapi_auth;
pub mod session_auth;

pub mod cors;
pub mod prometheus;

// pub mod template1;
// pub mod template2;
// pub mod template3;
