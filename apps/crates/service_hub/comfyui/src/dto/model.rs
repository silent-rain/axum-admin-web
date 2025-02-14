//! ComfyUI 模型管理

use serde::{Deserialize, Serialize};

use crate::api_clients::dto::{Embedding, Model};

/// 获取扩展节点列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EmbeddingsResp {
    pub data: Vec<Embedding>,
}

/// 获取 Checkpoint 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CheckpointsResp {
    pub data: Vec<Model>,
}

/// 获取 lora 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LorasResp {
    pub data: Vec<Model>,
}

/// 获取 vae 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VaesResp {
    pub data: Vec<Model>,
}

/// 获取 clip_vision 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClipVisionsResp {
    pub data: Vec<Model>,
}

/// 获取 controlnet 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ControlnetsResp {
    pub data: Vec<Model>,
}

/// 获取 upscale_model 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpscaleModelsResp {
    pub data: Vec<Model>,
}

/// 获取 ipadapter 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IpadaptersResp {
    pub data: Vec<Model>,
}

/// 获取 unet_gguf 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UnetGgufsResp {
    pub data: Vec<Model>,
}
