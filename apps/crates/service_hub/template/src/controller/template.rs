//! 模板管理

use crate::{
    dto::template::{
        BatchCreateAppTemplateReq, BatchCreateAppTemplateResp, BatchDeleteAppTemplateReq,
        BatchDeleteAppTemplateResp, CreateAppTemplateReq, CreateAppTemplateResp,
        DeleteAppTemplateReq, DeleteAppTemplateResp, GetAppTemplateReq, GetAppTemplateResp,
        GetAppTemplatesReq, GetAppTemplatesResp, UpdateAppTemplateReq, UpdateAppTemplateResp,
        UpdateAppTemplateStatusReq, UpdateAppTemplateStatusResp,
    },
    service::template::AppTemplateService,
};

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

/// 控制器
pub struct AppTemplateController;

impl AppTemplateController {
    /// 获取所有{{InterfaceName}}
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetAppTemplatesReq>,
    ) -> Responder<GetAppTemplatesResp> {
        let app_template_service: AppTemplateService = provider.provide();
        let (results, total) = app_template_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取{{InterfaceName}}信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetAppTemplateReq>,
    ) -> Responder<GetAppTemplateResp> {
        let app_template_service: AppTemplateService = provider.provide();
        let result = app_template_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加{{InterfaceName}}
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateAppTemplateReq>,
    ) -> Responder<CreateAppTemplateResp> {
        let app_template_service: AppTemplateService = provider.provide();
        let _result = app_template_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量添加{{InterfaceName}}
    pub async fn batch_create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<BatchCreateAppTemplateReq>,
    ) -> Responder<BatchCreateAppTemplateResp> {
        let app_template_service: AppTemplateService = provider.provide();
        let _result = app_template_service.batch_create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新{{InterfaceName}}
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateAppTemplateReq>,
    ) -> Responder<UpdateAppTemplateResp> {
        let app_template_service: AppTemplateService = provider.provide();
        let _result = app_template_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新{{InterfaceName}}状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateAppTemplateStatusReq>,
    ) -> Responder<UpdateAppTemplateStatusResp> {
        let app_template_service: AppTemplateService = provider.provide();
        let _result = app_template_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除{{InterfaceName}}
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteAppTemplateReq>,
    ) -> Responder<DeleteAppTemplateResp> {
        let app_template_service: AppTemplateService = provider.provide();
        let _result = app_template_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量删除{{InterfaceName}}
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<BatchDeleteAppTemplateReq>,
    ) -> Responder<BatchDeleteAppTemplateResp> {
        let app_template_service: AppTemplateService = provider.provide();
        let _result = app_template_service.batch_delete(req.ids.clone()).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
