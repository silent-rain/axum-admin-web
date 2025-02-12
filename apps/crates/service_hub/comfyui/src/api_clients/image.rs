//! 图片相关接口

use std::path::Path;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart::{Form, Part},
};
use uuid::Uuid;

use crate::dto::api_clients::{ImageViewReq, UploadImage, UploadMaskImage, UploadMaskImageReq};

use super::{client::ComfyUIClient, error::Error};

impl ComfyUIClient {
    /// 上传图片
    ///
    /// 上传图片到ComfyUI服务器的input目录
    pub async fn upload_image(&self, file_path: String) -> Result<UploadImage, Error> {
        let url = format!("{}/upload/image", self.base_api);

        // 文件处理
        let path = Path::new(&file_path);
        let extension = path
            .extension()
            .ok_or_else(|| Error::FileNameExtension("not found extension".to_string()))?
            .to_str()
            .ok_or_else(|| Error::ConvertType("OsStr convert str failed".to_string()))?;
        let file_bytes = tokio::fs::read(path).await?;
        let file_name = Uuid::new_v4().to_string();

        let part = Part::stream(file_bytes.clone()).file_name(format!("{file_name}.{extension}"));
        let form: Form = Form::new().part("image", part);

        let resp = self
            .client
            .post(url)
            .timeout(self.timeout)
            .multipart(form)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    /// 上传蒙版图片，一般用于局部重绘
    pub async fn upload_mask_image(
        &self,
        data: UploadMaskImageReq,
    ) -> Result<UploadMaskImage, Error> {
        let url = format!("{}/upload/mask", self.base_api);

        // 文件处理
        let path = Path::new(&data.image);
        let extension = path
            .extension()
            .ok_or_else(|| Error::FileNameExtension("not found extension".to_string()))?
            .to_str()
            .ok_or_else(|| Error::ConvertType("OsStr convert str failed".to_string()))?;
        let file_bytes = tokio::fs::read(path).await?;
        let file_name = Uuid::new_v4().to_string();

        let part = Part::stream(file_bytes.clone()).file_name(format!("{file_name}.{extension}"));
        let form: Form = Form::new()
            .text("type", data.r#type)
            .text("subfolder", data.subfolder)
            .text("original_ref", data.original_ref)
            .part("image", part);

        let resp = self
            .client
            .post(url)
            .timeout(self.timeout)
            .multipart(form)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    /// 图片的在线预览接口, 返回图片 Bytes 字节
    ///
    /// 上传图像，生图图像，蒙蔽图像，均通过该接口预览
    pub async fn view_image(&self, params: ImageViewReq) -> Result<Vec<u8>, Error> {
        let url = format!("{}/view", self.base_api);

        let mut headers = HeaderMap::new();
        headers.insert(
            "accept",
            HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;"),
        );

        let resp = self
            .client
            .get(url)
            .timeout(self.timeout)
            .headers(headers)
            .query(&params)
            .send()
            .await?
            .bytes()
            .await?;

        Ok(resp.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Ok;
    use tokio::io::AsyncWriteExt;

    use crate::dto::api_clients::ImageOriginalRef;

    const BASE_API: &str = "http://127.0.0.1:8188/api";

    #[tokio::test]
    async fn test_upload_image() {
        let client = ComfyUIClient::new(BASE_API.to_string());
        let results = client.upload_image("./assets/OIG3.jpeg".to_string()).await;
        println!("results: {:#?}", results);
    }

    #[tokio::test]
    async fn test_upload_mask_image() {
        let original_ref = ImageOriginalRef {
            filename: "7a721506-a062-431c-836e-16b19747df50.jpeg".to_string(),
            r#type: "".to_string(),
            subfolder: "input".to_string(),
        };

        let data = UploadMaskImageReq {
            image: "./assets/ComfyUI_00033.png".to_string(),
            r#type: "input".to_string(),
            subfolder: "clipspace".to_string(),
            original_ref: original_ref.to_string(),
        };

        let client = ComfyUIClient::new(BASE_API.to_string());
        let results = client.upload_mask_image(data).await;
        println!("results: {:#?}", results);
    }

    #[tokio::test]
    async fn test_view_image() -> anyhow::Result<()> {
        let params = ImageViewReq {
            filename: "ComfyUI_temp_ayiia_00001_.png".to_string(),
            r#type: "temp".to_string(),
            subfolder: "".to_string(),
        };
        let client = ComfyUIClient::new(BASE_API.to_string());
        let results = client.view_image(params).await?;

        println!("result empty: {:?}", results.is_empty());

        // 打开一个文件进行写操作，如果文件不存在则创建它
        let mut file = tokio::fs::File::create("./assets/view.png").await?;

        // 将二进制数据写入文件
        file.write_all(&results).await?;

        Ok(())
    }
}
