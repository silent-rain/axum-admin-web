//! 文件资源管理

use std::io::{Read, Write};

use crate::{
    dao::file_resource::FileResourceDao,
    dto::file_resource::{
        DeleteFileResourceReq, GetFileResourceReq, GetFileResourcesReq, ShowFileReq,
        UpdateFileResourceReq, UploadFileReq, UploadFilesReq,
    },
};

use code::{Error, ErrorMsg};
use entity::system::sys_file_resource;

use nject::injectable;
use sea_orm::Set;
use tracing::error;
use uuid::Uuid;

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
    ) -> Result<(Vec<sys_file_resource::Model>, u64), ErrorMsg> {
        let (results, total) = self.image_resource_dao.list(req).await.map_err(|err| {
            error!("查询文件列表失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询文件列表失败")
        })?;

        // 屏蔽文件内容
        // for item in results.iter_mut() {
        //     item.data = "".as_bytes().to_vec();
        // }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(
        &self,
        req: GetFileResourceReq,
    ) -> Result<sys_file_resource::Model, ErrorMsg> {
        let result = self
            .image_resource_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询文件信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询文件信息失败")
            })?
            .ok_or_else(|| {
                error!("文件不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("文件不存在")
            })?;

        Ok(result)
    }

    /// 通过hash值获取详情数据
    pub async fn info_by_hash(
        &self,
        req: ShowFileReq,
    ) -> Result<sys_file_resource::Model, ErrorMsg> {
        let result = self
            .image_resource_dao
            .info_by_hash(req.hash)
            .await
            .map_err(|err| {
                error!("获取文件失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("获取文件失败")
            })?
            .ok_or_else(|| {
                error!("文件不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("文件不存在")
            })?;

        Ok(result)
    }

    /// 上传文件
    pub async fn upload_file(
        &self,
        mut req: UploadFileReq,
    ) -> Result<sys_file_resource::Model, ErrorMsg> {
        let name = req.file.metadata.file_name.ok_or_else(|| {
            error!("请求参数异常");
            Error::RequestError("请求参数异常".to_string()).into_msg()
        })?;

        let extension = req
            .file
            .metadata
            .content_type
            .map_or("".to_owned(), |v| v.to_string());

        let buffer = vec![];
        req.file
            .contents
            .write_all(&buffer)
            .map_err(|err| Error::UploadFileError(err.to_string()))?;

        let img_size = req.file.contents.bytes().count() as u16;

        let hash = Uuid::new_v4().to_string().replace('-', "");

        let model = sys_file_resource::ActiveModel {
            name: Set(name),
            hash: Set(hash),
            data: Set(buffer),
            extension: Set(extension),
            size: Set(img_size),
            ..Default::default()
        };

        let result = self.image_resource_dao.create(model).await.map_err(|err| {
            error!("传文件信息失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("传文件信息失败")
        })?;

        Ok(result)
    }

    /// 批量上传文件
    pub async fn upload_files(&self, req: UploadFilesReq) -> Result<i32, ErrorMsg> {
        let mut models = Vec::new();
        for mut file in req.files {
            let name = file.metadata.file_name.ok_or_else(|| {
                error!("请求参数异常");
                Error::RequestError("请求参数异常".to_string()).into_msg()
            })?;

            let extension = file
                .metadata
                .content_type
                .map_or("".to_owned(), |v| v.to_string());

            let buffer = vec![];
            file.contents
                .write_all(&buffer)
                .map_err(|err| Error::UploadFileError(err.to_string()))?;

            let img_size = file.contents.bytes().count() as u16;

            let hash = Uuid::new_v4().to_string().replace('-', "");

            let model = sys_file_resource::ActiveModel {
                name: Set(name),
                hash: Set(hash),
                data: Set(buffer),
                extension: Set(extension),
                size: Set(img_size),
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
                Error::DbAddError.into_msg().with_msg("批量上传文件失败")
            })?;

        Ok(result)
    }

    /// 更新文件
    pub async fn update(&self, req: UpdateFileResourceReq) -> Result<u64, ErrorMsg> {
        let model = sys_file_resource::ActiveModel {
            id: Set(req.id),
            name: Set(req.name),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self.image_resource_dao.update(model).await.map_err(|err| {
            error!("更新文件失败, err: {:#?}", err);
            Error::DbUpdateError.into_msg().with_msg("更新文件失败")
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
                Error::DbDeleteError.into_msg().with_msg("删除文件信息失败")
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
                Error::DbBatchDeleteError
                    .into_msg()
                    .with_msg("批量删除文件信息失败")
            })?;

        Ok(result)
    }
}
