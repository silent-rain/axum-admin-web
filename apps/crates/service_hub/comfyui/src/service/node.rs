//! ComfyUI 节点管理

use code::{Error, ErrorMsg};
use log::error;
use nject::injectable;
use serde_json::Value;

use crate::{api_clients::client::ComfyUIClient, dto::node::ObjectInfoReq};

/// 服务层
#[injectable]
pub struct ComfyUINodeService {}

impl ComfyUINodeService {
    /// 获取 ComfyUI 客户端
    fn comfyui_client(&self) -> ComfyUIClient {
        ComfyUIClient::new()
        // .with_base_api(base_api)
    }

    /// 获取节点信息
    pub async fn object_info(&self, req: ObjectInfoReq) -> Result<Value, ErrorMsg> {
        let result = self
            .comfyui_client()
            .object_info(req.node_class.as_deref())
            .await
            .map_err(|err| {
                error!("获取节点信息失败, err: {err}");
                Error::ComfyUIError(err.to_string())
                    .into_msg()
                    .with_msg("获取节点信息失败")
            })?;

        Ok(result)
    }

    /// 获取扩展节点列表
    pub async fn extensions(&self) -> Result<(Vec<String>, u64), ErrorMsg> {
        let results = self.comfyui_client().extensions().await.map_err(|err| {
            error!("获取扩展节点列表失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取扩展节点列表失败")
        })?;

        let total = results.len() as u64;
        Ok((results, total))
    }
}
