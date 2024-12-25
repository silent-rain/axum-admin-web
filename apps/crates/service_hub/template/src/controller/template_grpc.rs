//! 模板GRPC示例

use crate::service::template::AppTemplateService;

use grpc_api::template::{
    app_template_service_server::AppTemplateService as AppTemplateGrpcService,
    BatchDeleteAppTemplateReq, BatchDeleteAppTemplateResp, CreateAppTemplateReq,
    CreateAppTemplateResp, DeleteAppTemplateReq, DeleteAppTemplateResp, GetAppTemplateReq,
    GetAppTemplateResp, LisAppTemplatesReq, LisAppTemplatesResp, UpdateAppTemplateReq,
    UpdateAppTemplateResp, UpdateAppTemplateStatusReq, UpdateAppTemplateStatusResp,
};
use nject::injectable;
use response::Response;
use tonic::{Request, Status};
use utils::json::struct_to_struct;

/// 控制器
#[injectable]
pub struct AppTemplateController {
    app_template_service: AppTemplateService,
}

#[tonic::async_trait]
impl AppTemplateGrpcService for AppTemplateController {
    async fn lis_app_templates(
        &self,
        req: Request<LisAppTemplatesReq>,
    ) -> Result<tonic::Response<LisAppTemplatesResp>, Status> {
        let req = struct_to_struct(&req.into_inner())
            .map_err(|err| tonic::Status::aborted(err.to_string()))?;
        let resp = self.app_template_service.list(req).await;
        match resp {
            Ok((results, total)) => Ok(Response::data_list(results, total)
                .to_pb::<LisAppTemplatesResp>()
                .unwrap()),
            Err(err) => Err(tonic::Status::aborted(err.msg())),
        }
    }

    async fn ge_app_template(
        &self,
        req: Request<GetAppTemplateReq>,
    ) -> Result<tonic::Response<GetAppTemplateResp>, Status> {
        let id = req.into_inner().id;
        let resp = self.app_template_service.info(id).await;
        match resp {
            Ok(results) => Ok(Response::data(results)
                .to_pb::<GetAppTemplateResp>()
                .unwrap()),
            Err(err) => Err(tonic::Status::aborted(err.msg())),
        }
    }

    async fn create_app_template(
        &self,
        req: Request<CreateAppTemplateReq>,
    ) -> Result<tonic::Response<CreateAppTemplateResp>, Status> {
        let data = struct_to_struct(&req.into_inner())
            .map_err(|err| tonic::Status::aborted(err.to_string()))?;

        let resp = self.app_template_service.add(data).await;
        match resp {
            Ok(_v) => Ok(Response::<()>::ok()
                .to_pb::<CreateAppTemplateResp>()
                .unwrap()),
            Err(err) => Err(tonic::Status::aborted(err.msg())),
        }
    }

    async fn update_app_template(
        &self,
        req: Request<UpdateAppTemplateReq>,
    ) -> Result<tonic::Response<UpdateAppTemplateResp>, Status> {
        let data: crate::dto::template::UpdateAppTemplateReq = struct_to_struct(&req.into_inner())
            .map_err(|err| tonic::Status::aborted(err.to_string()))?;
        let resp: Result<u64, code::ErrorMsg> = self.app_template_service.update(data).await;

        match resp {
            Ok(_v) => Ok(Response::<()>::ok()
                .to_pb::<UpdateAppTemplateResp>()
                .unwrap()),
            Err(err) => Err(tonic::Status::aborted(err.msg())),
        }
    }

    async fn update_app_template_status(
        &self,
        req: Request<UpdateAppTemplateStatusReq>,
    ) -> Result<tonic::Response<UpdateAppTemplateStatusResp>, Status> {
        let data: crate::dto::template::UpdateAppTemplateReq = struct_to_struct(&req.into_inner())
            .map_err(|err| tonic::Status::aborted(err.to_string()))?;

        let resp = self
            .app_template_service
            .status(data.id, data.status as i8)
            .await;

        match resp {
            Ok(_v) => Ok(Response::<()>::ok()
                .to_pb::<UpdateAppTemplateStatusResp>()
                .unwrap()),
            Err(err) => Err(tonic::Status::aborted(err.msg())),
        }
    }

    async fn delete_app_template(
        &self,
        req: Request<DeleteAppTemplateReq>,
    ) -> Result<tonic::Response<DeleteAppTemplateResp>, Status> {
        let id = req.into_inner().id;
        let resp = self.app_template_service.delete(id).await;
        match resp {
            Ok(_v) => Ok(Response::<()>::ok()
                .to_pb::<DeleteAppTemplateResp>()
                .unwrap()),
            Err(err) => Err(tonic::Status::aborted(err.msg())),
        }
    }

    async fn batch_delete_app_template(
        &self,
        req: Request<BatchDeleteAppTemplateReq>,
    ) -> Result<tonic::Response<BatchDeleteAppTemplateResp>, Status> {
        let ids = req.into_inner().ids;
        let resp = self.app_template_service.batch_delete(ids).await;
        match resp {
            Ok(_v) => Ok(Response::<()>::ok()
                .to_pb::<BatchDeleteAppTemplateResp>()
                .unwrap()),
            Err(err) => Err(tonic::Status::aborted(err.msg())),
        }
    }
}
