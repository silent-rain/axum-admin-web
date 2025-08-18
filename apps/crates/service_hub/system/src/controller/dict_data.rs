//! 字典数据管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::dict_data::{
        CreateDictDataReq, CreateDictDataResp, DeleteDictDataReq, DeleteDictDataResp,
        GetDictDataReq, GetDictDataResp, GetDictDatasReq, GetDictDatasResp, UpdateDictDataReq,
        UpdateDictDataResp, UpdateDictDataStatusReq, UpdateDictDataStatusResp,
    },
    service::dict_data::DictDataService,
};

/// 控制器
pub struct DictDataController;

impl DictDataController {
    /// 获取字典数据列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDictDatasReq>,
    ) -> Responder<GetDictDatasResp> {
        let dict_data_service: DictDataService = provider.provide();
        let (results, total) = dict_data_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取字典数据信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDictDataReq>,
    ) -> Responder<GetDictDataResp> {
        let dict_data_service: DictDataService = provider.provide();
        let result = dict_data_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加字典数据
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDictDataReq>,
    ) -> Responder<CreateDictDataResp> {
        let dict_data_service: DictDataService = provider.provide();
        let _result = dict_data_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新字典数据
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDictDataReq>,
    ) -> Responder<UpdateDictDataResp> {
        let dict_data_service: DictDataService = provider.provide();
        let _result = dict_data_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新字典数据状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDictDataStatusReq>,
    ) -> Responder<UpdateDictDataStatusResp> {
        let dict_data_service: DictDataService = provider.provide();
        dict_data_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除字典数据
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteDictDataReq>,
    ) -> Responder<DeleteDictDataResp> {
        let dict_data_service: DictDataService = provider.provide();
        let _result = dict_data_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
