//! 模板管理

use crate::{
    dto::template::{
        AddAppTemplateReq, BatchDeleteAppTemplateReq, GetAppTemplateListReq, UpdateAppTemplateReq,
        UpdateAppTemplateStatusReq,
    },
    service::template::AppTemplateService,
};

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Extension, Json,
};
use inject::AInjectProvider;
use response::Response;

/// 控制器
pub struct AppTemplateController;

impl AppTemplateController {
    /// 获取所有{{InterfaceName}}
    pub async fn all(provider: Extension<AInjectProvider>) -> impl IntoResponse {
        let perm_user_service: AppTemplateService = provider.provide();
        let resp = perm_user_service.all().await;
        match resp {
            Ok((results, total)) => Response::data_list(results, total),
            Err(err) => err.into(),
        }
    }

    /// 获取所有{{InterfaceName}}
    pub async fn list(
        provider: Extension<AInjectProvider>,
        Query(req): Query<GetAppTemplateListReq>,
    ) -> impl IntoResponse {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.list(req).await;
        match resp {
            Ok((results, total)) => Response::data_list(results, total),
            Err(err) => err.into(),
        }
    }

    /// 获取单个{{InterfaceName}}信息
    pub async fn info(provider: Extension<AInjectProvider>, id: Path<i32>) -> impl IntoResponse {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.info(*id).await;
        match resp {
            Ok(v) => Response::data(v),
            Err(err) => err.into(),
        }
    }

    /// 添加{{InterfaceName}}
    pub async fn add(
        provider: Extension<AInjectProvider>,
        Json(data): Json<AddAppTemplateReq>,
    ) -> impl IntoResponse {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.add(data).await;
        match resp {
            Ok(_v) => Response::<()>::ok(),
            Err(err) => err.into(),
        }
    }

    /// 更新{{InterfaceName}}
    pub async fn update(
        provider: Extension<AInjectProvider>,
        id: Path<i32>,
        Json(data): Json<UpdateAppTemplateReq>,
    ) -> impl IntoResponse {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.update(*id, data).await;
        match resp {
            Ok(_v) => Response::<()>::ok(),
            Err(err) => err.into(),
        }
    }

    /// 更新{{InterfaceName}}状态
    pub async fn status(
        provider: Extension<AInjectProvider>,
        id: Path<i32>,
        data: Json<UpdateAppTemplateStatusReq>,
    ) -> impl IntoResponse {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service
            .status(*id, data.status.clone() as i8)
            .await;
        match resp {
            Ok(_v) => Response::<()>::ok(),
            Err(err) => err.into(),
        }
    }

    /// 删除{{InterfaceName}}
    pub async fn delete(provider: Extension<AInjectProvider>, id: Path<i32>) -> impl IntoResponse {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.delete(*id).await;
        match resp {
            Ok(_v) => Response::<()>::ok(),
            Err(err) => err.into(),
        }
    }

    /// 批量删除{{InterfaceName}}
    pub async fn batch_delete(
        provider: Extension<AInjectProvider>,
        data: Json<BatchDeleteAppTemplateReq>,
    ) -> impl IntoResponse {
        let app_template_service: AppTemplateService = provider.provide();
        let resp = app_template_service.batch_delete(data.ids.clone()).await;
        match resp {
            Ok(_v) => Response::<()>::ok(),
            Err(err) => err.into(),
        }
    }
}
