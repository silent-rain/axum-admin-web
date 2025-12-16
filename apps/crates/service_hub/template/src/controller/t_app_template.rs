//! 模板管理

use crate::{
    dto::t_app_template::{
        BatchCreateTemplateReq, BatchDeleteTemplateReq, CreateTemplateReq, DeleteTemplateReq,
        GetTemplateReq, GetTemplateResp, GetTemplatesReq, GetTemplatesResp, UpdateTemplateReq,
        UpdateTemplateStatusReq,
    },
    service::t_app_template::TemplateService,
};

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

/// 控制器
pub struct TemplateController;

impl TemplateController {
    /// 获取所有{{InterfaceName}}
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetTemplatesReq>,
    ) -> Responder<GetTemplatesResp> {
        let app_template_service: TemplateService = provider.provide();
        let (results, total) = app_template_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取{{InterfaceName}}信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetTemplateReq>,
    ) -> Responder<GetTemplateResp> {
        let app_template_service: TemplateService = provider.provide();
        let result = app_template_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加{{InterfaceName}}
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateTemplateReq>,
    ) -> Responder<()> {
        let app_template_service: TemplateService = provider.provide();
        let _result = app_template_service.create(req).await?;

        Ok(Response::ok())
    }

    /// 批量添加{{InterfaceName}}
    pub async fn batch_create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<BatchCreateTemplateReq>,
    ) -> Responder<()> {
        let app_template_service: TemplateService = provider.provide();
        let _result = app_template_service.batch_create(req).await?;

        Ok(Response::ok())
    }

    /// 更新{{InterfaceName}}
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateTemplateReq>,
    ) -> Responder<()> {
        let app_template_service: TemplateService = provider.provide();
        let _result = app_template_service.update(req).await?;

        Ok(Response::ok())
    }

    /// 更新{{InterfaceName}}状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateTemplateStatusReq>,
    ) -> Responder<()> {
        let app_template_service: TemplateService = provider.provide();
        let _result = app_template_service.update_status(req).await?;

        Ok(Response::ok())
    }

    /// 删除{{InterfaceName}}
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteTemplateReq>,
    ) -> Responder<()> {
        let app_template_service: TemplateService = provider.provide();
        let _result = app_template_service.delete(req).await?;

        Ok(Response::ok())
    }

    /// 批量删除{{InterfaceName}}
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<BatchDeleteTemplateReq>,
    ) -> Responder<()> {
        let app_template_service: TemplateService = provider.provide();
        let _result = app_template_service.batch_delete(req.ids.clone()).await?;

        Ok(Response::ok())
    }
}
