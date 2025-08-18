//! ComfyUI 任务管理

use code::{Error, ErrorMsg};
use log::error;
use nject::injectable;

use crate::{
    api_clients::{
        client::ComfyUIClient,
        dto::{PromptResult, QueueRemaining, Queues},
    },
    dto::task::{DeleteQueueReq, History, HistoryReq, HistorysReq, PushPromptReq},
};

/// 服务层
#[injectable]
pub struct ComfyUITaskService {}

impl ComfyUITaskService {
    /// 获取 ComfyUI 客户端
    fn comfyui_client(&self) -> ComfyUIClient {
        ComfyUIClient::new()
        // TODO 待修复 从数据库中读取 base_api config 表 or user_config?
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
    pub async fn historys(&self, req: HistorysReq) -> Result<(Vec<History>, u64), ErrorMsg> {
        let result = self
            .comfyui_client()
            .historys(req.max_items)
            .await
            .map_err(|err| {
                error!("获取所有历史任务数据失败, err: {err}");
                Error::ComfyUIError(err.to_string())
                    .into_msg()
                    .with_msg("获取所有历史任务数据失败")
            })?;

        let mut imgs = Vec::<History>::new();
        for (prompt_id, raw_history) in result {
            let mut img = History {
                prompt_id,
                images: vec![],
            };
            for (_i, output) in raw_history.outputs {
                img.images.extend(output.images);
            }
            imgs.push(img);
        }

        let total = imgs.len() as u64;
        Ok((imgs, total))
    }

    /// 获取所有历史任务数据
    pub async fn history(&self, req: HistoryReq) -> Result<History, ErrorMsg> {
        let result = self
            .comfyui_client()
            .history(&req.prompt_id)
            .await
            .map_err(|err| {
                error!("获取所有历史任务数据失败, err: {err}");
                Error::ComfyUIError(err.to_string())
                    .into_msg()
                    .with_msg("获取所有历史任务数据失败")
            })?;

        let mut imgs = Vec::<History>::new();
        for (prompt_id, raw_history) in result {
            let mut img = History {
                prompt_id,
                images: vec![],
            };
            for (_i, output) in raw_history.outputs {
                img.images.extend(output.images);
            }
            imgs.push(img);
        }
        if imgs.is_empty() {
            return Ok(History {
                prompt_id: "".to_string(),
                images: vec![],
            });
        }
        Ok(imgs[0].clone())
    }

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
