//! ComfyUI 系统管理

use axum::Extension;
use axum_response::{Responder, Response};
use inject::AInjectProvider;

use crate::{dto::system::SystemStatsResp, service::system::ComfyUISystemService};

/// 控制器
pub struct ComfyUISystemController;

impl ComfyUISystemController {
    /// 获取系统统计信息
    pub async fn system_stats(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<SystemStatsResp> {
        let comfyui_system_service: ComfyUISystemService = provider.provide();
        let result = comfyui_system_service.system_stats().await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }
}
