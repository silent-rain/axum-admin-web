//! ComfyUI 系统管理

use err_code::{Error, ErrorMsg};
use log::error;
use nject::injectable;

use crate::api_clients::{client::ComfyUIClient, dto::SystemStats};

/// 服务层
#[injectable]
pub struct ComfyUISystemService {}

impl ComfyUISystemService {
    /// 获取 ComfyUI 客户端
    fn comfyui_client(&self) -> ComfyUIClient {
        ComfyUIClient::new()
        // .with_base_api(base_api)
    }

    /// 获取系统统计信息
    pub async fn system_stats(&self) -> Result<SystemStats, ErrorMsg> {
        let result = self.comfyui_client().system_stats().await.map_err(|err| {
            error!("获取系统统计信息失败, err: {err}");
            Error::ComfyUIError(err.to_string()).into_err_with_msg("获取系统统计信息失败")
        })?;

        Ok(result)
    }
}
