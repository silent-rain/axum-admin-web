//! 岗位管理

use crate::{
    dto::position::{
        CreatePositionReq, CreatePositionResp, DeletePositionReq, DeletePositionResp,
        GetPositionReq, GetPositionResp, GetPositionsReq, GetPositionsResp, UpdatePositionReq,
        UpdatePositionResp, UpdatePositionStatusReq, UpdatePositionStatusResp,
    },
    service::position::PositionService,
};

use axum::{extract::Query, Extension, Json};
use inject::AInjectProvider;
use response::{Responder, Response};

/// 控制器
pub struct PositionController;

impl PositionController {
    /// 获取岗位列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetPositionsReq>,
    ) -> Responder<GetPositionsResp> {
        let position_service: PositionService = provider.provide();
        let (results, total) = position_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取岗位信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetPositionReq>,
    ) -> Responder<GetPositionResp> {
        let position_service: PositionService = provider.provide();
        let result = position_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加岗位
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreatePositionReq>,
    ) -> Responder<CreatePositionResp> {
        let position_service: PositionService = provider.provide();
        let _result = position_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新岗位
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdatePositionReq>,
    ) -> Responder<UpdatePositionResp> {
        let position_service: PositionService = provider.provide();
        let _result = position_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新岗位状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdatePositionStatusReq>,
    ) -> Responder<UpdatePositionStatusResp> {
        let position_service: PositionService = provider.provide();
        position_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除岗位
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeletePositionReq>,
    ) -> Responder<DeletePositionResp> {
        let position_service: PositionService = provider.provide();
        let _result = position_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
