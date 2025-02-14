//! ComfyUI 图片管理

use axum_typed_multipart::FieldData;
use axum_typed_multipart::TryFromMultipart;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

pub use crate::api_clients::dto::ImageViewReq;
pub use crate::api_clients::dto::UploadMaskImageReq;
use crate::api_clients::dto::{UploadImage, UploadMaskImage};

/// 上传图片 请求体
#[derive(TryFromMultipart)]
pub struct UploadImageReq {
    /// 文件路径
    #[form_data(limit = "5MiB")]
    pub file: FieldData<NamedTempFile>,
}

/// 上传图片 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UploadImageResp {
    #[serde(flatten)]
    pub data: UploadImage,
}

/// 上传蒙版图片 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UploadMaskImageResp {
    #[serde(flatten)]
    pub data: UploadMaskImage,
}
