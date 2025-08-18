//! 模板GRPC示例

use grpc_api::template::{
    BatchDeleteAppTemplateReq, BatchDeleteAppTemplateResp, CreateAppTemplateReq,
    CreateAppTemplateResp, DeleteAppTemplateReq, DeleteAppTemplateResp, GetAppTemplateReq,
    GetAppTemplateResp, GetAppTemplatesReq, GetAppTemplatesResp, UpdateAppTemplateReq,
    UpdateAppTemplateResp, UpdateAppTemplateStatusReq, UpdateAppTemplateStatusResp,
    app_template_service_server::AppTemplateService as AppTemplateGrpcService,
};

use axum_response::Response;
use nject::injectable;
use tonic::{Request, Status};
use utils::json::struct_to_struct;

use crate::AppTemplateService;

/// 控制器
#[injectable]
pub struct AppTemplateController {
    app_template_service: AppTemplateService,
}

#[tonic::async_trait]
impl AppTemplateGrpcService for AppTemplateController {
    async fn get_app_templates(
        &self,
        req: Request<GetAppTemplatesReq>,
    ) -> Result<tonic::Response<GetAppTemplatesResp>, Status> {
        let req = struct_to_struct(&req.into_inner())
            .map_err(|err| tonic::Status::aborted(err.to_string()))?;
        let resp = self.app_template_service.list(req).await;
        match resp {
            Ok((results, total)) => Ok(Response::data_list(results, total)
                .to_pb::<GetAppTemplatesResp>()
                .unwrap()),
            Err(err) => Err(tonic::Status::aborted(err.msg())),
        }
    }

    async fn get_app_template(
        &self,
        req: Request<GetAppTemplateReq>,
    ) -> Result<tonic::Response<GetAppTemplateResp>, Status> {
        let req = struct_to_struct(&req.into_inner())
            .map_err(|err| tonic::Status::aborted(err.to_string()))?;
        let resp = self.app_template_service.info(req).await;
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

        let resp = self.app_template_service.create(data).await;
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
        let data: crate::dto::template::UpdateAppTemplateStatusReq =
            struct_to_struct(&req.into_inner())
                .map_err(|err| tonic::Status::aborted(err.to_string()))?;

        let resp = self.app_template_service.update_status(data).await;

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
        let data: crate::dto::template::DeleteAppTemplateReq = struct_to_struct(&req.into_inner())
            .map_err(|err| tonic::Status::aborted(err.to_string()))?;

        let resp = self.app_template_service.delete(data).await;
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
