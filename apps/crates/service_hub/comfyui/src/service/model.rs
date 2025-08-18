//! ComfyUI 模型管理

use code::{Error, ErrorMsg};
use log::error;
use nject::injectable;

use crate::api_clients::{
    client::ComfyUIClient,
    dto::{Embedding, Model},
};

/// 服务层
#[injectable]
pub struct ComfyUIModelService {}

impl ComfyUIModelService {
    /// 获取 ComfyUI 客户端
    fn comfyui_client(&self) -> ComfyUIClient {
        ComfyUIClient::new()
        // .with_base_api(base_api)
    }

    /// 获取连续的词向量模型列表
    pub async fn embeddings(&self) -> Result<(Vec<Embedding>, u64), ErrorMsg> {
        let results = self.comfyui_client().embeddings().await.map_err(|err| {
            error!("获取连续的词向量模型列表失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取连续的词向量模型列表失败")
        })?;

        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取 Checkpoint 模型列表
    pub async fn checkpoints(&self) -> Result<(Vec<Model>, u64), ErrorMsg> {
        let results = self.comfyui_client().checkpoints().await.map_err(|err| {
            error!("获取 Checkpoint 模型列表失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取 Checkpoint 模型列表失败")
        })?;

        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取 lora 模型列表
    pub async fn loras(&self) -> Result<(Vec<Model>, u64), ErrorMsg> {
        let results = self.comfyui_client().loras().await.map_err(|err| {
            error!("获取 lora 模型列表失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取 lora 模型列表失败")
        })?;

        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取 Vae 模型列表
    pub async fn vaes(&self) -> Result<(Vec<Model>, u64), ErrorMsg> {
        let results = self.comfyui_client().vaes().await.map_err(|err| {
            error!("获取 Vae 模型列表失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取 Vae 模型列表失败")
        })?;

        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取 clip_vision 模型列表
    pub async fn clip_visions(&self) -> Result<(Vec<Model>, u64), ErrorMsg> {
        let results = self.comfyui_client().clip_visions().await.map_err(|err| {
            error!("获取 clip_vision 模型列表失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取 clip_vision 模型列表失败")
        })?;

        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取 controlnet 模型列表
    pub async fn controlnets(&self) -> Result<(Vec<Model>, u64), ErrorMsg> {
        let results = self.comfyui_client().controlnets().await.map_err(|err| {
            error!("获取 controlnet 模型列表失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取 controlnet 模型列表失败")
        })?;

        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取 upscale_model 模型列表
    pub async fn upscale_models(&self) -> Result<(Vec<Model>, u64), ErrorMsg> {
        let results = self
            .comfyui_client()
            .upscale_models()
            .await
            .map_err(|err| {
                error!("获取 upscale_model 模型列表失败, err: {err}");
                Error::ComfyUIError(err.to_string())
                    .into_msg()
                    .with_msg("获取 upscale_model 模型列表失败")
            })?;

        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取 ipadapter 模型列表
    pub async fn ipadapters(&self) -> Result<(Vec<Model>, u64), ErrorMsg> {
        let results = self.comfyui_client().ipadapters().await.map_err(|err| {
            error!("获取 ipadapter 模型列表失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取 ipadapter 模型列表失败")
        })?;

        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取 unet_gguf 模型列表
    pub async fn unet_ggufs(&self) -> Result<(Vec<Model>, u64), ErrorMsg> {
        let results = self.comfyui_client().unet_ggufs().await.map_err(|err| {
            error!("获取 unet_gguf 模型列表失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取 unet_gguf 模型列表失败")
        })?;

        let total = results.len() as u64;
        Ok((results, total))
    }
}
