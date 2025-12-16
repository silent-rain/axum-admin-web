//! 后台管理 WEB 服务

use axum::response::Html;
use embed_asset::EmbedAssetTrait;
use embed_asset::web::AssetAdminWebDist;

use axum::body::Body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use log::info;
use log::warn;

/// 控制器
pub struct AdminWebSiteController;

impl AdminWebSiteController {
    /// 后台管理首页
    pub async fn index() -> impl IntoResponse {
        let filename = "index.html".to_string();
        info!("req filename: {filename}");

        let r = AssetAdminWebDist;
        let asset = r.data(&filename).unwrap_or_default();
        let mimetype = r.mimetype(&filename).unwrap_or_default();
        let content_type = format!("{mimetype}; charset=utf-8");

        Response::builder()
            .header("Content-Type", content_type)
            .status(StatusCode::OK)
            .body(Body::from(asset))
            .map_err(|e| (StatusCode::NOT_FOUND, format!("Not Found: {}", e)))
    }

    /// 后台管理静态资源
    pub async fn static_dir(Path(path): Path<String>) -> impl IntoResponse {
        let mut filename = path.clone();

        if filename.is_empty() || filename == "/" {
            filename = "index.html".to_string()
        }
        info!("req filename: {filename}");

        let r = AssetAdminWebDist;
        let data = match r.data(&filename) {
            Some(v) => v,
            None => {
                warn!("Not Found: {}", &filename);

                match r.data("index.html") {
                    Some(v) => v,
                    None => {
                        return Err((StatusCode::NOT_FOUND, "Not Found: index.html".to_string()));
                    }
                }
            }
        };

        let mimetype = r.mimetype(&filename).unwrap_or_default();
        let content_type = format!("{mimetype}; charset=utf-8");

        Response::builder()
            .header("Content-Type", content_type)
            .status(StatusCode::OK)
            .body(Body::from(data))
            .map_err(|e| (StatusCode::NOT_FOUND, format!("Not Found: {}", e)))
    }

    // 后台服务 - 失败示例
    pub async fn _index2(Path(path): Path<String>) -> Html<String> {
        let mut filename = path.clone();

        if filename.is_empty() || filename == "/" {
            filename = "index.html".to_string()
        }
        warn!("req filename: {filename}");
        let r = AssetAdminWebDist;

        let body = r.to_string(&filename).unwrap_or_else(|_| "".to_string());
        Html(body)
    }
}
