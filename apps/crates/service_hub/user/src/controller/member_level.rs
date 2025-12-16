//! 会员等级管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::member_level::{
        CreateMemberLevelReq, DeleteMemberLevelReq, GetMemberLevelReq, GetMemberLevelResp,
        GetMemberLevelsReq, GetMemberLevelsResp, UpdateMemberLevelReq, UpdateMemberLevelStatusReq,
    },
    service::member_level::MemberLevelService,
};

/// 控制器
pub struct MemberLevelController;

impl MemberLevelController {
    /// 获取会员等级列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetMemberLevelsReq>,
    ) -> Responder<GetMemberLevelsResp> {
        let member_level_service: MemberLevelService = provider.provide();
        let (results, total) = member_level_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取会员等级信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetMemberLevelReq>,
    ) -> Responder<GetMemberLevelResp> {
        let member_level_service: MemberLevelService = provider.provide();
        let result = member_level_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加会员等级
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateMemberLevelReq>,
    ) -> Responder<()> {
        let member_level_service: MemberLevelService = provider.provide();
        let _result = member_level_service.create(data).await?;

        Ok(Response::ok())
    }

    /// 更新会员等级
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateMemberLevelReq>,
    ) -> Responder<()> {
        let member_level_service: MemberLevelService = provider.provide();
        let _result = member_level_service.update(req).await?;

        Ok(Response::ok())
    }

    /// 更新会员等级状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateMemberLevelStatusReq>,
    ) -> Responder<()> {
        let member_level_service: MemberLevelService = provider.provide();
        member_level_service.update_status(req).await?;

        Ok(Response::ok())
    }

    /// 删除会员等级
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteMemberLevelReq>,
    ) -> Responder<()> {
        let member_level_service: MemberLevelService = provider.provide();
        let _result = member_level_service.delete(req).await?;

        Ok(Response::ok())
    }
}
