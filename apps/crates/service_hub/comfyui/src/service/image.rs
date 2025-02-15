//! ComfyUI 图片管理

use std::io::Read;

use axum_typed_multipart::FieldData;
use code::{Error, ErrorMsg};
use nject::injectable;
use tempfile::NamedTempFile;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::error;
use utils::file::file_extension;
use uuid::Uuid;

use crate::{
    api_clients::{
        client::ComfyUIClient,
        dto::{
            ImageOriginalRef, ImageViewReq, UploadImage, UploadMaskImage,
            UploadMaskImageReq as ApiUploadMaskImageReq,
        },
    },
    dto::image::{UploadImageAndMask, UploadImageAndMaskReq, UploadImageReq, UploadMaskImageReq},
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

    /// 将上传的图片写入到 `upload/images` 目录, 并返回文件路径
    async fn image_tmp_filepath(mut image: FieldData<NamedTempFile>) -> Result<String, ErrorMsg> {
        let file_name = image.metadata.file_name.ok_or_else(|| {
            error!("请求参数异常, image is empty");
            Error::RequestError("请求参数异常, image is empty".to_string()).into_msg()
        })?;
        let extension = file_extension(file_name.clone())?;

        let mut buffer = vec![];
        image
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

        Ok(filepath)
    }

    /// 上传图片
    pub async fn upload_image(&self, req: UploadImageReq) -> Result<UploadImage, ErrorMsg> {
        let filepath = Self::image_tmp_filepath(req.image).await?;

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
    pub async fn upload_mask(&self, req: UploadMaskImageReq) -> Result<UploadMaskImage, ErrorMsg> {
        error!("{:#?}", req.original_ref);
        let filepath = Self::image_tmp_filepath(req.image).await?;

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

    /// 同时上传图片与图片对应的蒙版, 一般用于局部重绘
    pub async fn upload_image_and_mask(
        &self,
        req: UploadImageAndMaskReq,
    ) -> Result<UploadImageAndMask, ErrorMsg> {
        let image_result = self
            .upload_image(UploadImageReq { image: req.image })
            .await?;

        let original_ref = ImageOriginalRef {
            filename: image_result.name.clone(),
            r#type: image_result.r#type.clone(),
            subfolder: image_result.subfolder.clone(),
        };
        let image_mask_result = self
            .upload_mask(UploadMaskImageReq {
                image: req.image_mask,
                r#type: "input".to_string(),
                subfolder: Some("mask".to_string()),
                original_ref: original_ref.to_string(),
            })
            .await?;

        Ok(UploadImageAndMask {
            image: image_result,
            image_mask: image_mask_result,
        })
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
