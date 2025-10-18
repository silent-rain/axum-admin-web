//! 文件资源管理

use std::io::Read;

use log::error;
use nject::injectable;
use sea_orm::Set;
use utils::file::file_extension;
use uuid::Uuid;

use entity::system::file_resource;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::file_resource::FileResourceDao,
    dto::file_resource::{
        DeleteFileResourceReq, GetFileResourceReq, GetFileResourcesReq, ShowImageReq,
        UpdateFileResourceReq, UploadFileReq, UploadFilesReq,
    },
};

/// 服务层
#[injectable]
pub struct FileResourceService {
    image_resource_dao: FileResourceDao,
}

impl FileResourceService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetFileResourcesReq,
    ) -> Result<(Vec<file_resource::Model>, u64), ErrorMsg> {
        let (results, total) = self.image_resource_dao.list(req).await.map_err(|err| {
            error!("查询文件列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询文件列表失败")
        })?;

        // 屏蔽文件内容
        // for item in results.iter_mut() {
        //     item.data = "".as_bytes().to_vec();
        // }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetFileResourceReq) -> Result<file_resource::Model, ErrorMsg> {
        let result = self
            .image_resource_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询文件信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询文件信息失败")
            })?
            .ok_or_else(|| {
                error!("文件不存在");
                Error::DbQueryEmptyError.into_err_with_msg("文件不存在")
            })?;

        Ok(result)
    }

    /// 通过hash值获取详情数据
    pub async fn info_by_hash(&self, req: ShowImageReq) -> Result<file_resource::Model, ErrorMsg> {
        let result = self
            .image_resource_dao
            .info_by_hash(req.hash)
            .await
            .map_err(|err| {
                error!("获取文件失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("获取文件失败")
            })?
            .ok_or_else(|| {
                error!("文件不存在");
                Error::DbQueryEmptyError.into_err_with_msg("文件不存在")
            })?;

        Ok(result)
    }

    /// 上传文件
    pub async fn upload_file(
        &self,
        mut req: UploadFileReq,
    ) -> Result<file_resource::Model, ErrorMsg> {
        let file_name = req.file.metadata.file_name.ok_or_else(|| {
            error!("请求参数异常");
            Error::RequestError("请求参数异常".to_string()).into_err()
        })?;

        let extension = file_extension(file_name.clone()).map_err(|e| e.into_err())?;

        let mut buffer = vec![];
        req.file
            .contents
            .read_to_end(&mut buffer)
            .map_err(|err| Error::UploadFileError(err.to_string()).into_err())?;

        let mut content_type = req
            .file
            .metadata
            .content_type
            .map_or("".to_owned(), |v| v.to_string());

        if content_type.is_empty() {
            let file_kind = infer::get(&buffer).ok_or(
                Error::HeaderContentType(format!("{file_name} file type is known")).into_err(),
            )?;
            content_type = file_kind.mime_type().to_string();
        }
        let file_size = buffer.len() as u16;

        let hash = Uuid::new_v4().to_string().replace('-', "");

        let model = file_resource::ActiveModel {
            file_name: Set(file_name),
            hash: Set(hash),
            data: Set(buffer),
            extension: Set(extension),
            content_type: Set(content_type),
            size: Set(file_size),
            ..Default::default()
        };

        let result = self.image_resource_dao.create(model).await.map_err(|err| {
            error!("传文件信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("传文件信息失败")
        })?;

        Ok(result)
    }

    /// 批量上传文件
    pub async fn upload_files(&self, req: UploadFilesReq) -> Result<i32, ErrorMsg> {
        let mut models = Vec::new();
        for mut file in req.files {
            let file_name = file.metadata.file_name.ok_or_else(|| {
                error!("请求参数异常");
                Error::RequestError("请求参数异常".to_string()).into_err()
            })?;

            let extension = file_extension(file_name.clone()).map_err(|e| e.into_err())?;

            let mut buffer = vec![];
            file.contents
                .read_to_end(&mut buffer)
                .map_err(|err| Error::UploadFileError(err.to_string()).into_err())?;

            let mut content_type = file
                .metadata
                .content_type
                .map_or("".to_owned(), |v| v.to_string());

            if content_type.is_empty() {
                let file_kind = infer::get(&buffer).ok_or(
                    Error::HeaderContentType(format!("{file_name} file type is known")).into_err(),
                )?;
                content_type = file_kind.mime_type().to_string();
            }

            let file_size = buffer.len() as u16;

            let hash = Uuid::new_v4().to_string().replace('-', "");

            let model = file_resource::ActiveModel {
                file_name: Set(file_name),
                hash: Set(hash),
                data: Set(buffer),
                extension: Set(extension),
                content_type: Set(content_type),
                size: Set(file_size),
                ..Default::default()
            };
            models.push(model);
        }

        let result = self
            .image_resource_dao
            .batch_create(models)
            .await
            .map_err(|err| {
                error!("批量上传文件失败, err: {:#?}", err);
                Error::DbAddError.into_err_with_msg("批量上传文件失败")
            })?;

        Ok(result)
    }

    /// 更新文件
    pub async fn update(&self, req: UpdateFileResourceReq) -> Result<u64, ErrorMsg> {
        let model = file_resource::ActiveModel {
            id: Set(req.id),
            file_name: Set(req.file_name),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self.image_resource_dao.update(model).await.map_err(|err| {
            error!("更新文件失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新文件失败")
        })?;

        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteFileResourceReq) -> Result<u64, ErrorMsg> {
        let result = self
            .image_resource_dao
            .delete(req.id)
            .await
            .map_err(|err| {
                error!("删除文件信息失败, err: {:#?}", err);
                Error::DbDeleteError.into_err_with_msg("删除文件信息失败")
            })?;

        Ok(result)
    }

    /// 批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, ErrorMsg> {
        let result = self
            .image_resource_dao
            .batch_delete(ids)
            .await
            .map_err(|err| {
                error!("批量删除文件信息失败, err: {:#?}", err);
                Error::DbBatchDeleteError.into_err_with_msg("批量删除文件信息失败")
            })?;

        Ok(result)
    }
}
