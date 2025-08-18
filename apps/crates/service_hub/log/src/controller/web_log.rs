//! WEB日志管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

use crate::{
    dto::web_log::{
        CreateWebLogReq, CreateWebLogResp, GetWebLogReq, GetWebLogResp, GetWebLogsReq,
        GetWebLogsResp,
    },
    service::web_log::WebLogService,
};

/// 控制器
pub struct WebLogController;

impl WebLogController {
    /// 获取WEB日志列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetWebLogsReq>,
    ) -> Responder<GetWebLogsResp> {
        let log_web_service: WebLogService = provider.provide();
        let (results, total) = log_web_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取WEB日志信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetWebLogReq>,
    ) -> Responder<GetWebLogResp> {
        let log_web_service: WebLogService = provider.provide();
        let result = log_web_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加WEB日志
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateWebLogReq>,
    ) -> Responder<CreateWebLogResp> {
        let log_web_service: WebLogService = provider.provide();
        let _result = log_web_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
