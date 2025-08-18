//! 字典维度管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::dict_dimension::{
        CreateDictDimensionReq, CreateDictDimensionResp, DeleteDictDimensionReq,
        DeleteDictDimensionResp, GetDictDimensionReq, GetDictDimensionResp, GetDictDimensionsReq,
        GetDictDimensionsResp, UpdateDictDimensionReq, UpdateDictDimensionResp,
        UpdateDictDimensionStatusReq, UpdateDictDimensionStatusResp,
    },
    service::dict_dimension::DictDimensionService,
};

/// 控制器
pub struct DictDimensionController;

impl DictDimensionController {
    /// 获取字典维度列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDictDimensionsReq>,
    ) -> Responder<GetDictDimensionsResp> {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let (results, total) = dict_dimension_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取字典维度信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDictDimensionReq>,
    ) -> Responder<GetDictDimensionResp> {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let result = dict_dimension_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加字典维度
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDictDimensionReq>,
    ) -> Responder<CreateDictDimensionResp> {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let _result = dict_dimension_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新字典维度
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDictDimensionReq>,
    ) -> Responder<UpdateDictDimensionResp> {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let _result = dict_dimension_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新字典维度状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDictDimensionStatusReq>,
    ) -> Responder<UpdateDictDimensionStatusResp> {
        let dict_dimension_service: DictDimensionService = provider.provide();
        dict_dimension_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除字典维度
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteDictDimensionReq>,
    ) -> Responder<DeleteDictDimensionResp> {
        let dict_dimension_service: DictDimensionService = provider.provide();
        let _result = dict_dimension_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
