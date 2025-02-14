//! ComfyUI 图片管理

use axum::extract::Query;
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use axum::Extension;
use axum_response::{Responder, Response, ResponseErr};
use axum_typed_multipart::TypedMultipart;
use axum_validator::Json;
use inject::AInjectProvider;

use crate::dto::image::{
    ImageViewReq, UploadImageReq, UploadImageResp, UploadMaskImageReq, UploadMaskImageResp,
};
use crate::service::image::ComfyUIImageService;

/// 控制器
pub struct ComfyUIImageController;

impl ComfyUIImageController {
    /// 上传图片
    pub async fn upload_image(
        Extension(provider): Extension<AInjectProvider>,
        TypedMultipart(req): TypedMultipart<UploadImageReq>,
    ) -> Responder<UploadImageResp> {
        let comfyui_image_service: ComfyUIImageService = provider.provide();
        let result = comfyui_image_service.upload_image(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 上传蒙版图片, 一般用于局部重绘
    pub async fn upload_mask_image(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UploadMaskImageReq>,
        // TypedMultipart(req): TypedMultipart<UploadFileReq>,
    ) -> Responder<UploadMaskImageResp> {
        let comfyui_image_service: ComfyUIImageService = provider.provide();
        let result = comfyui_image_service.upload_mask_image(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取图片, 返回图片
    pub async fn view_image(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<ImageViewReq>,
    ) -> Result<(HeaderMap, Vec<u8>), ResponseErr> {
        let comfyui_image_service: ComfyUIImageService = provider.provide();
        let bytes = comfyui_image_service.view_image(req).await?;

        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("image/jpeg"),
        );

        Ok((headers, bytes))
    }
}
