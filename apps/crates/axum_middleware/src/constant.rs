//! 常量

/// 系统鉴权标识
pub const AUTHORIZATION: &str = "Authorization";
/// 系统鉴权标识-前缀
pub const AUTHORIZATION_BEARER: &str = "Bearer ";

/// OPEN API鉴权标识
pub const OPENAPI_AUTHORIZATION: &str = "X-SR-Token";
/// OPEN API鉴权口令
pub const OPENAPI_PASSPHRASE: &str = "X-SR-Passphrase";

/// 请求白名单
pub const AUTH_WHITE_LIST: [&str; 6] = [
    "/health",
    "/auth/captcha",
    "/auth/login",
    "/auth/register",
    "/initialize/table",
    "/template/axum-validators/say-hello",
];
