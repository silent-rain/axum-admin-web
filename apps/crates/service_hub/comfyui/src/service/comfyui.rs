//! ComfyUI 服务

use code::{Error, ErrorMsg};
use nject::injectable;
use tracing::error;

use crate::{
    api_clients::{
        client::ComfyUIClient,
        dto::{PromptResult, QueueRemaining, Queues},
    },
    dto::comfyui::{DeleteQueueReq, HistoryReq, HistoryResp, PushPromptReq},
};

/*
全局配置:
- base_api
- 超时时间
- 模型列表
- lora列表

过程数据是否入库呢? 便于回溯? 服务器端,重启服务后历史信息会丢失
*/

/// 服务层
#[injectable]
pub struct ComfyUIService {}

impl ComfyUIService {
    /// 获取 ComfyUI 客户端
    fn comfyui_client(&self) -> ComfyUIClient {
        ComfyUIClient::new()
        // .with_base_api(base_api)
    }

    /// 发布绘图任务
    ///
    /// TODO 数据入库?
    pub async fn push_prompt(&self, req: PushPromptReq) -> Result<PromptResult, ErrorMsg> {
        let result = self.comfyui_client().prompt(req).await.map_err(|err| {
            error!("发布绘图任务失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("发布绘图任务失败")
        })?;

        Ok(result)
    }

    /// 获取服务器当前剩余任务列队数量
    pub async fn queue_remaining(&self) -> Result<QueueRemaining, ErrorMsg> {
        let result = self
            .comfyui_client()
            .queue_remaining()
            .await
            .map_err(|err| {
                error!("获取服务器当前剩余任务列队数量失败, err: {err}");
                Error::ComfyUIError(err.to_string())
                    .into_msg()
                    .with_msg("获取服务器当前剩余任务列队数量失败")
            })?;

        Ok(result)
    }

    /// 获取所有历史任务数据
    ///
    /// TODO 优化接口返回
    // pub async fn history(&self, req: HistoryReq) -> Result<HistoryResp, ErrorMsg> {
    //     let result = self
    //         .comfyui_client()
    //         .history(req.prompt_id.as_deref())
    //         .await
    //         .map_err(|err| {
    //             error!("获取所有历史任务数据失败, err: {err}");
    //             Error::ComfyUIError(err.to_string())
    //                 .into_msg()
    //                 .with_msg("获取所有历史任务数据失败")
    //         })?;

    //     Ok(result)
    // }

    /// 获取所有的队列
    pub async fn queues(&self) -> Result<Queues, ErrorMsg> {
        let result = self.comfyui_client().queues().await.map_err(|err| {
            error!("获取所有的队列失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("获取所有的队列失败")
        })?;

        Ok(result)
    }

    /// 清空队列
    pub async fn clear_queue(&self) -> Result<bool, ErrorMsg> {
        let result = self.comfyui_client().clear_queue().await.map_err(|err| {
            error!("清空队列失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("清空队列失败")
        })?;

        Ok(result)
    }

    /// 删除队列
    pub async fn delete_queue(&self, req: DeleteQueueReq) -> Result<bool, ErrorMsg> {
        let result = self
            .comfyui_client()
            .delete_queue(req.prompt_ids)
            .await
            .map_err(|err| {
                error!("删除队列失败, err: {err}");
                Error::ComfyUIError(err.to_string())
                    .into_msg()
                    .with_msg("删除队列失败")
            })?;

        Ok(result)
    }

    /// 取消当前任务
    pub async fn interrupt(&self) -> Result<bool, ErrorMsg> {
        let result = self.comfyui_client().interrupt().await.map_err(|err| {
            error!("取消当前任务失败, err: {err}");
            Error::ComfyUIError(err.to_string())
                .into_msg()
                .with_msg("取消当前任务失败")
        })?;

        Ok(result)
    }
}
