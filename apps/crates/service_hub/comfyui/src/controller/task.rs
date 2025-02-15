//! ComfyUI 任务管理

use axum::Extension;
use axum_response::{Responder, Response};
use axum_validator::{Json, Query};
use inject::AInjectProvider;

use crate::dto::task::{
    DeleteQueueReq, HistoryReq, HistoryResp, HistorysReq, HistorysResp, PushPromptReq,
    PushPromptResp, QueueRemainingResp, QueuesResp,
};
use crate::ComfyUITaskService;

/// 控制器
pub struct ComfyUITaskController;

impl ComfyUITaskController {
    /// 发布绘图任务
    pub async fn push_prompt(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<PushPromptReq>,
    ) -> Responder<PushPromptResp> {
        let comfyui_task_service: ComfyUITaskService = provider.provide();
        let result = comfyui_task_service.push_prompt(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取服务器当前剩余任务列队的数量
    ///
    /// 同时包含正在运行的任务
    pub async fn queue_remaining(
        Extension(provider): Extension<AInjectProvider>,
    ) -> Responder<QueueRemainingResp> {
        let comfyui_task_service: ComfyUITaskService = provider.provide();
        let result = comfyui_task_service.queue_remaining().await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取所有历史任务数据
    pub async fn historys(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<HistorysReq>,
    ) -> Responder<HistorysResp> {
        let comfyui_task_service: ComfyUITaskService = provider.provide();
        let (result, total) = comfyui_task_service.historys(req).await?;

        let resp = Response::data_list(result, total).to_json()?;
        Ok(resp)
    }

    /// 获取指定历史任务数据
    pub async fn history(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<HistoryReq>,
    ) -> Responder<HistoryResp> {
        let comfyui_task_service: ComfyUITaskService = provider.provide();
        let result = comfyui_task_service.history(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取所有的队列
    pub async fn queues(Extension(provider): Extension<AInjectProvider>) -> Responder<QueuesResp> {
        let comfyui_task_service: ComfyUITaskService = provider.provide();
        let result = comfyui_task_service.queues().await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 清空队列
    pub async fn clear_queue(Extension(provider): Extension<AInjectProvider>) -> Responder<()> {
        let comfyui_task_service: ComfyUITaskService = provider.provide();
        let _result = comfyui_task_service.clear_queue().await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除队列
    pub async fn delete_queue(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteQueueReq>,
    ) -> Responder<()> {
        let comfyui_task_service: ComfyUITaskService = provider.provide();
        let _result = comfyui_task_service.delete_queue(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 取消当前任务
    pub async fn interrupt(Extension(provider): Extension<AInjectProvider>) -> Responder<()> {
        let comfyui_task_service: ComfyUITaskService = provider.provide();
        let _result = comfyui_task_service.interrupt().await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
