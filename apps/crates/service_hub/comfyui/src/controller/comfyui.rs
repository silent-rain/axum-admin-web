//! ComfyUI 服务

use axum::Extension;
use axum_response::{Responder, Response};
use axum_validator::Query;
use inject::AInjectProvider;

use crate::dto::comfyui::{
    DeleteQueueReq, HistoryReq, HistoryResp, PushPromptReq, PushPromptResp, QueueRemainingResp,
    QueuesResp,
};
use crate::service::comfyui::ComfyUIService;

/// 控制器
pub struct ComfyUIController;

impl ComfyUIController {
    /// 发布绘图任务
    pub async fn push_prompt(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<PushPromptReq>,
    ) -> Responder<PushPromptResp> {
        let comfy_uiservice: ComfyUIService = provider.provide();
        let result = comfy_uiservice.push_prompt(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取服务器当前剩余任务列队的数量
    pub async fn queue_remaining(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<QueueRemainingResp> {
        let comfy_uiservice: ComfyUIService = provider.provide();
        let result = comfy_uiservice.queue_remaining().await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取所有历史任务数据
    // pub async fn history(
    //     Extension(provider): Extension<AInjectProvider>,
    //     Query(req): Query<HistoryReq>,
    // ) -> Responder<HistoryResp> {
    //     let comfy_uiservice: ComfyUIService = provider.provide();
    //     let result = comfy_uiservice.history(req).await?;

    //     let resp = Response::data(result).to_json()?;
    //     Ok(resp)
    // }

    /// 获取所有的队列
    pub async fn queues(Extension(provider): Extension<AInjectProvider>) -> Responder<QueuesResp> {
        let comfy_uiservice: ComfyUIService = provider.provide();
        let result = comfy_uiservice.queues().await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 清空队列
    pub async fn clear_queue(Extension(provider): Extension<AInjectProvider>) -> Responder<()> {
        let comfy_uiservice: ComfyUIService = provider.provide();
        let _result = comfy_uiservice.clear_queue().await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除队列
    pub async fn delete_queue(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<DeleteQueueReq>,
    ) -> Responder<()> {
        let comfy_uiservice: ComfyUIService = provider.provide();
        let _result = comfy_uiservice.delete_queue(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 取消当前任务
    pub async fn interrupt(Extension(provider): Extension<AInjectProvider>) -> Responder<()> {
        let comfy_uiservice: ComfyUIService = provider.provide();
        let _result = comfy_uiservice.interrupt().await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
