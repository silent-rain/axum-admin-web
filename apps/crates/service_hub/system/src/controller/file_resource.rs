//! 文件资源管理

use crate::{
    constant::HEADERS_X_IMG,
    dto::file_resource::{
        BatchDeleteFileResourceReq, BatchDeleteFileResourceResp, DeleteFileResourceReq,
        DeleteFileResourceResp, GetFileResourceReq, GetFileResourceResp, GetFileResourcesReq,
        GetFileResourcesResp, ShowImageReq, UpdateFileResourceReq, UpdateFileResourceResp,
        UploadFileReq, UploadFileResp, UploadFilesReq, UploadFilesResp,
    },
    service::file_resource::FileResourceService,
};

use axum::{body::Body, extract::Query, Extension, Json};
use axum_response::{Responder, Response, ResponseErr};
use axum_typed_multipart::TypedMultipart;
use code::Error;
use inject::AInjectProvider;

/// 控制器
pub struct FileResourceController;

impl FileResourceController {
    /// 获取文件列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetFileResourcesReq>,
    ) -> Responder<GetFileResourcesResp> {
        let image_resource_service: FileResourceService = provider.provide();
        let (results, total) = image_resource_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取文件信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetFileResourceReq>,
    ) -> Responder<GetFileResourceResp> {
        let image_resource_service: FileResourceService = provider.provide();
        let result = image_resource_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 更新文件
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateFileResourceReq>,
    ) -> Responder<UpdateFileResourceResp> {
        let image_resource_service: FileResourceService = provider.provide();
        let _result = image_resource_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除文件
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteFileResourceReq>,
    ) -> Responder<DeleteFileResourceResp> {
        let image_resource_service: FileResourceService = provider.provide();
        let _result = image_resource_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量删除文件
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<BatchDeleteFileResourceReq>,
    ) -> Responder<BatchDeleteFileResourceResp> {
        let image_resource_service: FileResourceService = provider.provide();
        let _result = image_resource_service.batch_delete(req.ids.clone()).await?;

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
        let image_resource_service: FileResourceService = provider.provide();
        let _result = image_resource_service.upload_file(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量上传文件
    pub async fn upload_files(
        Extension(provider): Extension<AInjectProvider>,
        TypedMultipart(req): TypedMultipart<UploadFilesReq>,
    ) -> Responder<UploadFilesResp> {
        let image_resource_service: FileResourceService = provider.provide();
        let _result = image_resource_service.upload_files(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 通过hash值获取图片
    /// TODO 待验证
    pub async fn show_image(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<ShowImageReq>,
    ) -> Result<axum::response::Response<Body>, ResponseErr> {
        let image_resource_service: FileResourceService = provider.provide();
        let result = image_resource_service.info_by_hash(req).await?;

        let img = result.data.to_vec();

        let resp = axum::response::Response::builder()
            .header(HEADERS_X_IMG, "true")
            .body(Body::from(img))
            .map_err(|err| Error::InternalServer(err.to_string()).into_msg())?;
        Ok(resp)
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
