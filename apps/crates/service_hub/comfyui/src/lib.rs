//! ComfyUI 服务管理

pub mod api_clients;
pub mod dto;

pub(crate) mod dao;

pub(crate) mod service;
pub use service::system::ComfyUISystemService;
pub use service::task::ComfyUITaskService;

pub(crate) mod controller;
pub use controller::system::ComfyUISystemController;
pub use controller::task::ComfyUITaskController;

pub(crate) mod router;
pub use router::task::ComfyUITaskRouter;
pub use router::ComfyUIRouter;

/*
全局配置:
- base_api
- 超时时间
- 模型列表
- lora列表

过程数据是否入库呢? 便于回溯? 服务器端,重启服务后历史信息会丢失
*/
