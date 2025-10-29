//! ComfyUI 模型管理

use axum::Extension;
use axum_response::{Responder, Response};
use inject::AInjectProvider;

use crate::{
    dto::model::{
        CheckpointsResp, ClipVisionsResp, ControlnetsResp, EmbeddingsResp, IpadaptersResp,
        LorasResp, UnetGgufsResp, UpscaleModelsResp, VaesResp,
    },
    service::model::ComfyUIModelService,
};

/// 控制器
pub struct ComfyUIModelController;

impl ComfyUIModelController {
    /// 获取连续的词向量模型列表
    pub async fn embeddings(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<EmbeddingsResp> {
        let comfyui_model_service: ComfyUIModelService = provider.provide();
        let (results, total) = comfyui_model_service.embeddings().await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取 Checkpoint 模型列表
    pub async fn checkpoints(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<CheckpointsResp> {
        let comfyui_model_service: ComfyUIModelService = provider.provide();
        let (results, total) = comfyui_model_service.checkpoints().await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取 lora 模型列表
    pub async fn loras(Extension(provider): Extension<AInjectProvider>) -> Responder<LorasResp> {
        let comfyui_model_service: ComfyUIModelService = provider.provide();
        let (results, total) = comfyui_model_service.loras().await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取 vae 模型列表
    pub async fn vaes(Extension(provider): Extension<AInjectProvider>) -> Responder<VaesResp> {
        let comfyui_model_service: ComfyUIModelService = provider.provide();
        let (results, total) = comfyui_model_service.vaes().await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取 clip_vision 模型列表
    pub async fn clip_visions(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<ClipVisionsResp> {
        let comfyui_model_service: ComfyUIModelService = provider.provide();
        let (results, total) = comfyui_model_service.clip_visions().await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取 controlnet 模型列表
    pub async fn controlnets(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<ControlnetsResp> {
        let comfyui_model_service: ComfyUIModelService = provider.provide();
        let (results, total) = comfyui_model_service.controlnets().await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取 upscale_model 模型列表
    pub async fn upscale_models(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<UpscaleModelsResp> {
        let comfyui_model_service: ComfyUIModelService = provider.provide();
        let (results, total) = comfyui_model_service.upscale_models().await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取 ipadapter 模型列表
    pub async fn ipadapters(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<IpadaptersResp> {
        let comfyui_model_service: ComfyUIModelService = provider.provide();
        let (results, total) = comfyui_model_service.ipadapters().await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取 unet_gguf 模型列表
    pub async fn unet_ggufs(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<UnetGgufsResp> {
        let comfyui_model_service: ComfyUIModelService = provider.provide();
        let (results, total) = comfyui_model_service.unet_ggufs().await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }
}
