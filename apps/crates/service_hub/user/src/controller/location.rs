//! 用户地理位置管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::location::{
        CreateLocationReq, CreateLocationResp, DeleteLocationReq, DeleteLocationResp,
        GetLocationReq, GetLocationResp, GetLocationsReq, GetLocationsResp, UpdateLocationReq,
        UpdateLocationResp,
    },
    service::location::LocationService,
};

/// 控制器
pub struct LocationController;

impl LocationController {
    /// 获取用户地理位置列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetLocationsReq>,
    ) -> Responder<GetLocationsResp> {
        let location_service: LocationService = provider.provide();
        let (results, total) = location_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取用户地理位置信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetLocationReq>,
    ) -> Responder<GetLocationResp> {
        let location_service: LocationService = provider.provide();
        let result = location_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加用户地理位置
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateLocationReq>,
    ) -> Responder<CreateLocationResp> {
        let location_service: LocationService = provider.provide();
        let _result = location_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新用户地理位置
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateLocationReq>,
    ) -> Responder<UpdateLocationResp> {
        let location_service: LocationService = provider.provide();
        let _result = location_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除用户地理位置
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteLocationReq>,
    ) -> Responder<DeleteLocationResp> {
        let location_service: LocationService = provider.provide();
        let _result = location_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
