//! 图片资源管理

use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use validator::Validate;

use entity::system::sys_image_resource;

/// 获取图片列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetImageResourcesReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 图片名称
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetImageResourcesResp {
    pub data_list: Vec<sys_image_resource::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetImageResourceReq {
    /// 图片ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetImageResourceResp {
    #[serde(flatten)]
    data: sys_image_resource::Model,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct ShowImageReq {
    /// 图片hash值
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowImageResp {
    #[serde(flatten)]
    data: sys_image_resource::Model,
}

/// 更新图片 请求体
#[derive(Default, Deserialize, Serialize, Validate)]
pub struct UpdateImageResourceReq {
    /// 图片ID
    pub id: i32,
    /// 图片名称
    pub name: String,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateImageResourceResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteImageResourceReq {
    /// 图片ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteImageResourceResp {}

/// 批量删除图片 请求体
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteImageResourceReq {
    /// ID列表
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteImageResourceResp {}

#[derive(TryFromMultipart)]
pub struct UploadFileReq {
    // The `unlimited arguments` means that this field will be limited to the
    // total size of the request body. If you want to limit the size of this
    // field to a specific value you can also specify a limit in bytes, like
    // '5MiB' or '1GiB' or 'unlimited'.
    #[form_data(limit = "5MiB")]
    pub file: FieldData<NamedTempFile>,

    // This field will be limited to the default size of 1MiB.
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFileResp {}

/// 多文件上传 请求体
#[derive(TryFromMultipart)]
pub struct UploadFilesReq {
    #[form_data(limit = "5MiB")]
    pub files: Vec<FieldData<NamedTempFile>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFilesResp {}
