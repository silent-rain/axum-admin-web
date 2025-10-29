//! 库表初始化

use crate::{
    dto::table::{CreateTableReq, CreateTableResp},
    service::table::TableService,
};

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json};
use inject::AInjectProvider;

/// 控制器
pub struct TableController;

impl TableController {
    /// 初始化库表
    pub async fn table(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateTableReq>,
    ) -> Responder<CreateTableResp> {
        let table_service: TableService = provider.provide();
        let _result = table_service.table(req).await?;

        let resp = Response::ok().with_msg("初始化成功");
        Ok(resp)
    }
}
