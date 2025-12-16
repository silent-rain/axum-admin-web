//! 用户信息管理

use axum_context::Context;
use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use log::warn;

use inject::AInjectProvider;

use crate::{
    dto::user_base::{
        CreateUserBaseReq, DeleteUserBaseReq, GetCheckUsernameReq, GetUserBaseReq, GetUserBasesReq,
        ProfileResp, RolesReq, UpdateUserBaseReq, UpdateUserBaseStatusReq,
    },
    service::user_base::UserBaseService,
};

/// 控制器
pub struct UserBaseController;

impl UserBaseController {
    /// 获取用户信息列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetUserBasesReq>,
    ) -> Responder<()> {
        let user_base_service: UserBaseService = provider.provide();
        let (results, total) = user_base_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取用户信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetUserBaseReq>,
    ) -> Responder<()> {
        let user_base_service: UserBaseService = provider.provide();
        let result = user_base_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加用户
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateUserBaseReq>,
    ) -> Responder<()> {
        let user_base_service: UserBaseService = provider.provide();
        let _result = user_base_service.create(data).await?;

        Ok(Response::ok())
    }

    /// 更新用户
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateUserBaseReq>,
    ) -> Responder<()> {
        let user_base_service: UserBaseService = provider.provide();
        user_base_service.update(req).await?;

        Ok(Response::ok())
    }

    /// 更新用户信息状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateUserBaseStatusReq>,
    ) -> Responder<()> {
        let user_base_service: UserBaseService = provider.provide();
        user_base_service.update_status(req).await?;

        Ok(Response::ok())
    }

    /// 删除用户
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteUserBaseReq>,
    ) -> Responder<()> {
        let user_base_service: UserBaseService = provider.provide();
        let _result = user_base_service.delete(req).await?;

        Ok(Response::ok())
    }
}

impl UserBaseController {
    /// 检查用户名称是否存在
    pub async fn check_username(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetCheckUsernameReq>,
    ) -> Responder<()> {
        let user_base_service: UserBaseService = provider.provide();
        user_base_service.check_username(req).await?;

        Ok(Response::ok())
    }

    /// 获取用户信息个人信息
    pub async fn profile(
        Extension(provider): Extension<AInjectProvider>,
        ctx: Context,
    ) -> Responder<ProfileResp> {
        let user_id = ctx.get_user_id();
        let username = ctx.get_user_name();
        warn!("profile context user_id: {user_id} username: {username}");

        let user_base_service: UserBaseService = provider.provide();
        let result = user_base_service.profile(user_id).await?;

        let resp = Response::data(result);
        Ok(resp)
    }

    /// 通过用户信息ID获角色色列表
    pub async fn roles(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<RolesReq>,
    ) -> Responder<()> {
        let user_base_service: UserBaseService = provider.provide();
        let (results, total) = user_base_service.roles(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }
}
