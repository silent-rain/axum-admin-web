//! ComfyUI 接口

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 系统信息
#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    pub os: String,
    pub ram_total: u64,
    pub ram_free: u64,
    pub comfyui_version: String,
    pub python_version: String,
    pub pytorch_version: String,
    pub embedded_python: bool,
    pub argv: Vec<String>,
}

/// 设备信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub r#type: String,
    pub index: u32,
    pub vram_total: u64,
    pub vram_free: u64,
    pub torch_vram_total: u64,
    pub torch_vram_free: u64,
}

/// 系统统计信息 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub system: System,
    pub devices: Vec<Device>,
}

/// 图片引用
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ImageOriginalRef {
    pub filename: String,  // 文件名称
    pub r#type: String,    // 上传图片的目标文件夹， "input"
    pub subfolder: String, // 上传图片的目标子文件夹， clipspace/pasted
}

impl std::fmt::Display for ImageOriginalRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{{"filename":"{}","subfolder":"{}","type":"{}"}}"#,
            self.filename, self.subfolder, self.r#type
        )
    }
}

/// 上传图片 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UploadImage {
    pub name: String,      // 文件名称
    pub r#type: String,    // 上传图片的目标文件夹， "input"
    pub subfolder: String, // 上传图片的目标子文件夹， clipspace/pasted
}

/// 上传蒙版图片 请求体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UploadMaskImageReq {
    pub image: String,        // 文件名称
    pub r#type: String,       // 上传图片的目标文件夹， "input"
    pub subfolder: String,    // 上传图片的目标子文件夹， clipspace/pasted
    pub original_ref: String, // 原图引用
}

/// 上传蒙版图片 响应体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UploadMaskImage {
    pub name: String,      // 文件名称
    pub r#type: String,    // 上传图片的目标文件夹， "input"
    pub subfolder: String, // 上传图片的目标子文件夹， clipspace/pasted
}

/// 图片预览 请求体
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ImageViewReq {
    pub filename: String, // 文件名称
    pub r#type: String,   // 上传图片的目标文件夹， "input"
    // pub rand: f32,         // 随机数
    pub subfolder: String, // 上传图片的目标子文件夹， clipspace/pasted
}

/// 历史任务输出的图片结构
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Image {
    pub filename: String,  // 文件名称
    pub r#type: String,    // 上传图片的目标文件夹， "input"
    pub subfolder: String, // 上传图片的目标子文件夹， clipspace/pasted
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Output {
    pub images: Vec<Image>,
}

/// 历史任务数据
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct History {
    pub prompt: Vec<Value>,
    pub outputs: HashMap<String, Output>,
    pub status: Value,
    pub meta: Value,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExecInfo {
    pub queue_remaining: i32,
}

/// 获取服务器当前剩余任务列队的数量
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueueRemaining {
    pub exec_info: ExecInfo,
}

/// 绘图任务的下发接口
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PromptReq {
    pub client_id: String,
    pub prompt: Value,
}

/// 绘图任务的下发接口
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PromptResult {
    pub prompt_id: String,
    pub number: i32,
    pub node_errors: Value,
}

/// 队列列表
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueueList {
    /// 当前正在运行的节点队列
    pub queue_running: Vec<Value>,
    /// 待执行的工作流队列
    pub queue_pending: Vec<Value>,
}
