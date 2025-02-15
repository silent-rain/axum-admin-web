//! ComfyUI 图片管理

use std::io::Read;

use code::{Error, ErrorMsg};
use nject::injectable;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::error;
use utils::file::file_extension;
use uuid::Uuid;

use crate::{
    api_clients::{
        client::ComfyUIClient,
        dto::{
            ImageViewReq, UploadImage, UploadMaskImage, UploadMaskImageReq as ApiUploadMaskImageReq,
        },
    },
    dto::image::{UploadImageReq, UploadMaskImageReq},
};

/// 服务层
#[injectable]
pub struct ComfyUIImageService {}

impl ComfyUIImageService {
    /// 获取 ComfyUI 客户端
    fn comfyui_client(&self) -> ComfyUIClient {
        ComfyUIClient::new()
        // .with_base_api(base_api)
    }

    /// 上传图片
    pub async fn upload_image(&self, mut req: UploadImageReq) -> Result<UploadImage, ErrorMsg> {
        let file_name = req.image.metadata.file_name.ok_or_else(|| {
            error!("请求参数异常, file_name is empty");
            Error::RequestError("请求参数异常, file_name is empty".to_string()).into_msg()
        })?;
        let extension = file_extension(file_name.clone())?;

        let mut buffer = vec![];
        req.image
            .contents
            .read_to_end(&mut buffer)
            .map_err(|err| Error::UploadFileError(err.to_string()))?;

        let file_hash_name = Uuid::new_v4().to_string();
        let filepath = format!("./upload/images/{file_hash_name}.{extension}");

        // 写入临时目录
        File::create(filepath.clone())
            .await
            .map_err(|err| {
                error!("获取文件实例失败, err: {err}");
                Error::Io(err).into_msg()
            })?
            .write_all(&buffer)
            .await
            .map_err(|err| {
                error!("写入文件失败, err: {err}");
                Error::Io(err).into_msg()
            })?;

        let result = self
            .comfyui_client()
            .upload_image(filepath)
            .await
            .map_err(|err| {
                error!("上传图片失败, err: {err}");
                Error::ComfyUIError(err.to_string())
                    .into_msg()
                    .with_msg("上传图片失败")
            })?;

        Ok(result)
    }

    /// 上传蒙版图片, 一般用于局部重绘
    pub async fn upload_mask_image(
        &self,
        mut req: UploadMaskImageReq,
    ) -> Result<UploadMaskImage, ErrorMsg> {
        let file_name = req.image.metadata.file_name.ok_or_else(|| {
            error!("请求参数异常, file_name is empty");
            Error::RequestError("请求参数异常, file_name is empty".to_string()).into_msg()
        })?;
        let extension = file_extension(file_name.clone())?;

        let mut buffer = vec![];
        req.image
            .contents
            .read_to_end(&mut buffer)
            .map_err(|err| Error::UploadFileError(err.to_string()))?;

        let file_hash_name = Uuid::new_v4().to_string();
        let filepath = format!("./upload/images/{file_hash_name}.{extension}");

        // 写入临时目录
        File::create(filepath.clone())
            .await
            .map_err(|err| {
                error!("获取文件实例失败, err: {err}");
                Error::Io(err).into_msg()
            })?
            .write_all(&buffer)
            .await
            .map_err(|err| {
                error!("写入文件失败, err: {err}");
                Error::Io(err).into_msg()
            })?;

        let data = ApiUploadMaskImageReq {
            image: filepath,
            r#type: req.r#type,
            subfolder: req.subfolder,
            original_ref: req.original_ref,
        };

        let result = self
            .comfyui_client()
            .upload_mask_image(data)
            .await
            .map_err(|err| {
                error!("上传蒙版图片失败, err: {err}");
                Error::ComfyUIError(err.to_string())
                    .into_msg()
                    .with_msg("上传蒙版图片失败")
            })?;

        Ok(result)
    }

    /// 获取图片
    pub async fn view_image(&self, req: ImageViewReq) -> Result<Vec<u8>, ErrorMsg> {
        let result = self.comfyui_client().view_image(req).await.map_err(|err| {
            error!("获取图片失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取图片失败")
        })?;

        Ok(result)
    }
}
