//! 中间件
pub mod constant;
pub mod error;

pub mod api_operation_log;
pub mod api_operation_log_fn;
pub mod casbin_auth;
pub mod openapi_auth;
pub mod system_api_auth;

pub mod cors;
pub mod prometheus;

// pub mod template1;
// pub mod template2;
// pub mod template3;
