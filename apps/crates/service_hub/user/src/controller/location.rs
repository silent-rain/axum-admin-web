//! 用户地理位置管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::location::{
        CreateLocationReq, DeleteLocationReq, GetLocationReq, GetLocationResp, GetLocationsReq,
        GetLocationsResp, UpdateLocationReq,
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
    ) -> Responder<()> {
        let location_service: LocationService = provider.provide();
        let _result = location_service.create(req).await?;

        Ok(Response::ok())
    }

    /// 更新用户地理位置
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateLocationReq>,
    ) -> Responder<()> {
        let location_service: LocationService = provider.provide();
        let _result = location_service.update(req).await?;

        Ok(Response::ok())
    }

    /// 删除用户地理位置
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteLocationReq>,
    ) -> Responder<()> {
        let location_service: LocationService = provider.provide();
        let _result = location_service.delete(req).await?;

        Ok(Response::ok())
    }
}
