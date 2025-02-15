//! ComfyUI 图片管理

use axum_typed_multipart::FieldData;
use axum_typed_multipart::TryFromMultipart;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

pub use crate::api_clients::dto::ImageViewReq;
use crate::api_clients::dto::{UploadImage, UploadMaskImage};

/// 上传图片 请求体
#[derive(TryFromMultipart)]
pub struct UploadImageReq {
    /// 文件路径
    #[form_data(limit = "5MiB")]
    pub image: FieldData<NamedTempFile>,
}

/// 上传图片 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UploadImageResp {
    #[serde(flatten)]
    pub data: UploadImage,
}

/// 上传蒙版图片 请求体
#[derive(TryFromMultipart)]
pub struct UploadMaskImageReq {
    /// 文件路径
    #[form_data(limit = "5MiB")]
    pub image: FieldData<NamedTempFile>,
    pub r#type: String,            // 上传图片的目标文件夹,  "input"
    pub subfolder: Option<String>, // 上传图片的目标子文件夹,  clipspace/pasted
    pub original_ref: String,      // 原图引用
}

/// 上传蒙版图片 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UploadMaskImageResp {
    #[serde(flatten)]
    pub data: UploadMaskImage,
}
