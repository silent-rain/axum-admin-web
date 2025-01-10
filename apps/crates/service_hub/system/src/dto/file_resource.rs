//! 文件资源管理

use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use validator::Validate;

/// 文件资源
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct FileResource {
    /// 文件ID
    pub id: i32,
    /// 文件名称
    pub file_name: String,
    /// 文件HASH值
    pub hash: String,
    /// 文件文件扩展名, 如svg, png
    pub extension: String,
    /// 内容类型, text/html
    pub content_type: String,
    /// 文件大小
    pub size: u16,
    /// 描述信息
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: String,
}

/// 获取文件列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetFileResourcesReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 文件名称
    pub file_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFileResourcesResp {
    pub data_list: Vec<FileResource>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetFileResourceReq {
    /// 文件ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFileResourceResp {
    #[serde(flatten)]
    data: FileResource,
}

/// 更新文件 请求体
#[derive(Default, Deserialize, Serialize, Validate)]
pub struct UpdateFileResourceReq {
    /// 文件ID
    pub id: i32,
    /// 文件名称
    pub file_name: String,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFileResourceResp {}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteFileResourceReq {
    /// 文件ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteFileResourceResp {}

/// 批量删除文件 请求体
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteFileResourceReq {
    /// ID列表
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteFileResourceResp {}

/// 单文件上传 请求体
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
    /// 文件列表
    #[form_data(limit = "5MiB")]
    pub files: Vec<FieldData<NamedTempFile>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFilesResp {}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct ShowImageReq {
    /// 文件hash值
    pub hash: String,
}
