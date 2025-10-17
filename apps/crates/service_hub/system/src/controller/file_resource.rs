//! 文件资源管理

use axum::{
    body::Body,
    http::{HeaderMap, HeaderName, HeaderValue, header},
};
use axum_response::{Responder, Response, ResponseErr};
use axum_typed_multipart::TypedMultipart;
use axum_validator::{Extension, Json, Query};

use err_code::Error;
use inject::AInjectProvider;

use crate::{
    dto::file_resource::{
        BatchDeleteFileResourceReq, BatchDeleteFileResourceResp, DeleteFileResourceReq,
        DeleteFileResourceResp, GetFileResourceReq, GetFileResourceResp, GetFileResourcesReq,
        GetFileResourcesResp, ShowImageReq, UpdateFileResourceReq, UpdateFileResourceResp,
        UploadFileReq, UploadFileResp, UploadFilesReq, UploadFilesResp,
    },
    service::file_resource::FileResourceService,
};

/// 控制器
pub struct FileResourceController;

impl FileResourceController {
    /// 获取文件列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetFileResourcesReq>,
    ) -> Responder<GetFileResourcesResp> {
        let file_resource_service: FileResourceService = provider.provide();
        let (results, total) = file_resource_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取文件信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetFileResourceReq>,
    ) -> Responder<GetFileResourceResp> {
        let file_resource_service: FileResourceService = provider.provide();
        let result = file_resource_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 更新文件
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateFileResourceReq>,
    ) -> Responder<UpdateFileResourceResp> {
        let file_resource_service: FileResourceService = provider.provide();
        let _result = file_resource_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除文件
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteFileResourceReq>,
    ) -> Responder<DeleteFileResourceResp> {
        let file_resource_service: FileResourceService = provider.provide();
        let _result = file_resource_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量删除文件
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<BatchDeleteFileResourceReq>,
    ) -> Responder<BatchDeleteFileResourceResp> {
        let file_resource_service: FileResourceService = provider.provide();
        let _result = file_resource_service.batch_delete(req.ids.clone()).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

impl FileResourceController {
    /// 上传文件
    pub async fn upload_file(
        Extension(provider): Extension<AInjectProvider>,
        TypedMultipart(req): TypedMultipart<UploadFileReq>,
    ) -> Responder<UploadFileResp> {
        let file_resource_service: FileResourceService = provider.provide();
        let _result = file_resource_service.upload_file(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量上传文件
    pub async fn upload_files(
        Extension(provider): Extension<AInjectProvider>,
        TypedMultipart(req): TypedMultipart<UploadFilesReq>,
    ) -> Responder<UploadFilesResp> {
        let file_resource_service: FileResourceService = provider.provide();
        let _result = file_resource_service.upload_files(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 下载文件
    pub async fn download(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<ShowImageReq>,
    ) -> Result<axum::response::Response<Body>, ResponseErr> {
        let filename = req.hash.clone();

        let file_resource_service: FileResourceService = provider.provide();
        let result = file_resource_service.info_by_hash(req).await?;

        let file_bytes = result.data;

        // 文件名称
        let content_disposition = format!(
            "attachment; filename=\"{:?}.{:?}\"",
            filename, result.extension
        );

        let resp = axum::response::Response::builder()
            .header("Content-Type", result.content_type)
            .header(header::CONTENT_DISPOSITION, content_disposition)
            .body(Body::from(file_bytes))
            .map_err(|err| Error::InternalServer(err.to_string()).into_err())?;
        Ok(resp)
    }

    /// 通过hash值获取图片
    /// 返回图片
    pub async fn show_image(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<ShowImageReq>,
    ) -> Result<(HeaderMap, Vec<u8>), ResponseErr> {
        let file_resource_service: FileResourceService = provider.provide();
        let result = file_resource_service.info_by_hash(req).await?;

        let file_bytes = result.data;

        // TODO 添加图片类型校验

        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("image/jpeg"),
        );

        Ok((headers, file_bytes))
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[test]
    fn test_uuid() {
        let uuid = Uuid::new_v4().to_string();
        assert_eq!(uuid.len(), 36);
    }
}
