//! ComfyUI 节点管理

use axum::Extension;
use axum_response::{Responder, Response};
use axum_validator::Query;
use inject::AInjectProvider;
use serde_json::Value;

use crate::{
    dto::node::{ExtensionsResp, ObjectInfoReq},
    service::node::ComfyUINodeService,
};

/// 控制器
pub struct ComfyUINodeController;

impl ComfyUINodeController {
    /// 获取节点信息
    pub async fn object_info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<ObjectInfoReq>,
    ) -> Responder<Value> {
        let comfyui_node_service: ComfyUINodeService = provider.provide();
        let result = comfyui_node_service.object_info(req).await?;

        let resp = Response::data(result);
        Ok(resp)
    }

    /// 获取扩展节点列表
    pub async fn extensions(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<ExtensionsResp> {
        let comfyui_node_service: ComfyUINodeService = provider.provide();
        let (results, total) = comfyui_node_service.extensions().await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }
}
