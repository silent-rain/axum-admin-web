//! ComfyUI 模型管理

use serde::{Deserialize, Serialize};

use crate::api_clients::dto::{Embedding, Model};

/// 获取扩展节点列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EmbeddingsResp {
    pub data_list: Vec<Embedding>,
    pub total: u64,
}

/// 获取 Checkpoint 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CheckpointsResp {
    pub data_list: Vec<Model>,
    pub total: u64,
}

/// 获取 lora 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LorasResp {
    pub data_list: Vec<Model>,
    pub total: u64,
}

/// 获取 vae 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VaesResp {
    pub data_list: Vec<Model>,
    pub total: u64,
}

/// 获取 clip_vision 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClipVisionsResp {
    pub data_list: Vec<Model>,
    pub total: u64,
}

/// 获取 controlnet 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ControlnetsResp {
    pub data_list: Vec<Model>,
    pub total: u64,
}

/// 获取 upscale_model 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpscaleModelsResp {
    pub data_list: Vec<Model>,
    pub total: u64,
}

/// 获取 ipadapter 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IpadaptersResp {
    pub data_list: Vec<Model>,
    pub total: u64,
}

/// 获取 unet_gguf 模型列表 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UnetGgufsResp {
    pub data_list: Vec<Model>,
    pub total: u64,
}
