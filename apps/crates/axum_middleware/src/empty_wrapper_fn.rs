//! 空的中间件包装
//! 用于规避 `tower_http::limit::RequestBodyLimitLayer` 中间件类型不匹配的错误

use axum::{extract::Request, middleware::Next, response::IntoResponse};

use axum_response::ResponseErr;

/// 空的中间件包装
/// ```ignore
/// use axum::Router;
///
/// Router::new().layer(axum::middleware::from_fn(empty_wrapper_layer))
/// ```
pub async fn empty_wrapper_layer(
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, ResponseErr> {
    // 响应
    let resp = next.run(request).await;

    Ok(resp)
}
