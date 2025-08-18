//! 职级管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::rank::{
        CreateRankReq, CreateRankResp, DeleteRankReq, DeleteRankResp, GetRankReq, GetRankResp,
        GetRanksReq, GetRanksResp, UpdateRankReq, UpdateRankResp, UpdateRankStatusReq,
        UpdateRankStatusResp,
    },
    service::rank::RankService,
};

/// 控制器
pub struct RankController;

impl RankController {
    /// 获取职级列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetRanksReq>,
    ) -> Responder<GetRanksResp> {
        let rank_service: RankService = provider.provide();
        let (results, total) = rank_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取职级信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetRankReq>,
    ) -> Responder<GetRankResp> {
        let department_service: RankService = provider.provide();
        let result = department_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加职级
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateRankReq>,
    ) -> Responder<CreateRankResp> {
        let department_service: RankService = provider.provide();
        let _result = department_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新职级
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateRankReq>,
    ) -> Responder<UpdateRankResp> {
        let department_service: RankService = provider.provide();
        let _result = department_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新职级状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateRankStatusReq>,
    ) -> Responder<UpdateRankStatusResp> {
        let department_service: RankService = provider.provide();
        department_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除职级
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteRankReq>,
    ) -> Responder<DeleteRankResp> {
        let department_service: RankService = provider.provide();
        let _result = department_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
