//! 配置管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::config::{
        CreateConfigReq, CreateConfigResp, DeleteConfigReq, DeleteConfigResp, GetConfigReq,
        GetConfigResp, GetConfigTreeReq, GetConfigTreeResp, GetConfigsReq, GetConfigsResp,
        UpdateConfigReq, UpdateConfigResp, UpdateConfigStatusReq, UpdateConfigStatusResp,
    },
    service::config::ConfigService,
};

/// 控制器
pub struct ConfigController;

impl ConfigController {
    /// 获配置列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetConfigsReq>,
    ) -> Responder<GetConfigsResp> {
        let config_service: ConfigService = provider.provide();
        let (results, total) = config_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取配置树列表
    pub async fn tree(
        Extension(provider): Extension<AInjectProvider>,
        Query(_req): Query<GetConfigTreeReq>,
    ) -> Responder<GetConfigTreeResp> {
        let config_service: ConfigService = provider.provide();
        let result = config_service.tree().await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取配置信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetConfigReq>,
    ) -> Responder<GetConfigResp> {
        let config_service: ConfigService = provider.provide();
        let result = config_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加配置
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateConfigReq>,
    ) -> Responder<CreateConfigResp> {
        let config_service: ConfigService = provider.provide();
        let _result = config_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新配置
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateConfigReq>,
    ) -> Responder<UpdateConfigResp> {
        let config_service: ConfigService = provider.provide();
        let _result = config_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新配置状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateConfigStatusReq>,
    ) -> Responder<UpdateConfigStatusResp> {
        let config_service: ConfigService = provider.provide();
        config_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除配置
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteConfigReq>,
    ) -> Responder<DeleteConfigResp> {
        let config_service: ConfigService = provider.provide();
        let _result = config_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
