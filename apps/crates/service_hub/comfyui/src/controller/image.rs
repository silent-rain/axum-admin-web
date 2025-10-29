//! ComfyUI 图片管理

use axum::Extension;
use axum::extract::Query;
use axum::http::{HeaderMap, HeaderName, HeaderValue};
use axum_response::{Responder, Response, ResponseErr};
use axum_typed_multipart::TypedMultipart;
use inject::AInjectProvider;

use crate::dto::image::UploadImageAndMaskResp;
use crate::{
    dto::image::{
        ImageViewReq, UploadImageAndMaskReq, UploadImageReq, UploadImageResp, UploadMaskImageReq,
        UploadMaskImageResp,
    },
    service::image::ComfyUIImageService,
};

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

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 上传蒙版图片, 一般用于局部重绘
    pub async fn upload_mask(
        Extension(provider): Extension<AInjectProvider>,
        TypedMultipart(req): TypedMultipart<UploadMaskImageReq>,
    ) -> Responder<UploadMaskImageResp> {
        let comfyui_image_service: ComfyUIImageService = provider.provide();
        let result = comfyui_image_service.upload_mask(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 同时上传图片与图片对应的蒙版, 一般用于局部重绘
    pub async fn upload_image_and_mask(
        Extension(provider): Extension<AInjectProvider>,
        TypedMultipart(req): TypedMultipart<UploadImageAndMaskReq>,
    ) -> Responder<UploadImageAndMaskResp> {
        let comfyui_image_service: ComfyUIImageService = provider.provide();
        let result = comfyui_image_service.upload_image_and_mask(req).await?;

        let resp = Response::data(result.into());
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
