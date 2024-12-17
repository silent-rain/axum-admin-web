//! 用户信息管理

use crate::{
    dto::user_base::{
        AddUserBaseReq, GetUserBaserListReq, UpdateUserBaseReq, UpdateUserBaseStatusReq,
    },
    service::user_base::UserBaseService,
};

use actix_validator::{Json, Query};
use context::Context;
use inject::AInjectProvider;
use response::Response;

use actix_web::{
    web::{Data, Path},
    Responder,
};
use tracing::warn;

/// 控制器
pub struct UserBaseController;

impl UserBaseController {
    /// 获取用户信息列表
    pub async fn list(
        provider: Data<AInjectProvider>,
        req: Query<GetUserBaserListReq>,
    ) -> impl Responder {
        let user_base_service: UserBaseService = provider.provide();
        let resp = user_base_service.list(req.into_inner()).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }

    /// 获取用户信息
    pub async fn info(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_base_service: UserBaseService = provider.provide();
        let resp = user_base_service.info(*id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 添加用户信息
    pub async fn add(
        provider: Data<AInjectProvider>,
        data: Json<AddUserBaseReq>,
    ) -> impl Responder {
        let user_base_service: UserBaseService = provider.provide();
        let resp = user_base_service.add(data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户信息
    pub async fn update(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateUserBaseReq>,
    ) -> impl Responder {
        let user_base_service: UserBaseService = provider.provide();
        let resp = user_base_service.update(*id, data.into_inner()).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户分享码
    pub async fn update_share_code(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
    ) -> impl Responder {
        let user_base_service: UserBaseService = provider.provide();
        let resp = user_base_service.update_share_code(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 更新用户信息状态
    pub async fn status(
        provider: Data<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateUserBaseStatusReq>,
    ) -> impl Responder {
        let user_base_service: UserBaseService = provider.provide();
        let resp = user_base_service
            .status(*id, data.status.clone() as i8)
            .await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }

    /// 删除用户信息
    pub async fn delete(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_base_service: UserBaseService = provider.provide();
        let resp = user_base_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::ok(),
            Err(err) => Response::err(err),
        }
    }
}

impl UserBaseController {
    /// 获取用户信息个人信息
    pub async fn profile(ctx: Context, provider: Data<AInjectProvider>) -> impl Responder {
        let user_id = ctx.get_user_id();
        let username = ctx.get_user_name();
        let request_id = ctx.get_request_id();
        warn!("profile context request_id: {request_id} user_id: {user_id} username: {username}");

        let user_base_service: UserBaseService = provider.provide();
        let resp = user_base_service.profile(user_id).await;
        match resp {
            Ok(v) => Response::ok().data(v),
            Err(err) => Response::err(err),
        }
    }

    /// 通过用户信息ID获角色色列表
    pub async fn roles(provider: Data<AInjectProvider>, id: Path<i32>) -> impl Responder {
        let user_base_service: UserBaseService = provider.provide();
        let resp = user_base_service.roles(*id).await;
        match resp {
            Ok((results, total)) => Response::ok().data_list(results, total),
            Err(err) => Response::err(err),
        }
    }
}
