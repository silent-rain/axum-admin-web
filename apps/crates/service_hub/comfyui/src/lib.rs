//! ComfyUI 服务管理

pub mod api_clients;
pub mod dto;
pub mod entity;
pub mod enums;

pub(crate) mod dao;

pub(crate) mod service;
pub use service::{
    image::ComfyUIImageService, model::ComfyUIModelService, node::ComfyUINodeService,
    system::ComfyUISystemService, task::ComfyUITaskService,
};

pub(crate) mod controller;
pub use controller::{
    image::ComfyUIImageController, model::ComfyUIModelController, node::ComfyUINodeController,
    system::ComfyUISystemController, task::ComfyUITaskController,
};

pub(crate) mod router;
pub use router::{
    ComfyUIRouter, image::ComfyUIImageRouter, model::ComfyUIModelRouter, node::ComfyUINodeRouter,
    system::ComfyUISystemRouter, task::ComfyUITaskRouter,
};

/*
全局配置:
- base_api
- 超时时间
- 模型列表
- lora列表

过程数据是否入库呢? 便于回溯? 服务器端,重启服务后历史信息会丢失
*/
