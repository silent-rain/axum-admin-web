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
        let resp = self.app_template_service.all().await;

        /*
         let data = LisAppTemplatesResp {
            code: todo!(),
            msg: todo!(),
            data: todo!(),
        };
        Ok(Response::new(data))
         */

        match resp {
            Ok((results, total)) => Ok(Response::data_list(results, total)
                .to_pb::<LisAppTemplatesResp>()
                .unwrap()),
            Err(err) => Err(tonic::Status::aborted("message")),
        }
    }

    async fn ge_app_template(
        &self,
        req: Request<GetAppTemplateReq>,
    ) -> Result<tonic::Response<GetAppTemplateResp>, Status> {
        let data = GetAppTemplateResp {
            code: todo!(),
            msg: todo!(),
            data: todo!(),
        };
        Ok(Response::new(data))
    }

    async fn create_app_template(
        &self,
        req: Request<CreateAppTemplateReq>,
    ) -> Result<tonic::Response<CreateAppTemplateResp>, Status> {
        let data = CreateAppTemplateResp {
            code: todo!(),
            msg: todo!(),
        };
        Ok(Response::new(data))
    }

    async fn update_app_template(
        &self,
        req: Request<UpdateAppTemplateReq>,
    ) -> Result<tonic::Response<UpdateAppTemplateResp>, Status> {
        let data = UpdateAppTemplateResp {
            code: todo!(),
            msg: todo!(),
        };
        Ok(Response::new(data))
    }

    async fn update_app_template_status(
        &self,
        req: Request<UpdateAppTemplateStatusReq>,
    ) -> Result<tonic::Response<UpdateAppTemplateStatusResp>, Status> {
        let data = UpdateAppTemplateStatusResp {
            code: todo!(),
            msg: todo!(),
        };
        Ok(Response::new(data))
    }

    async fn delete_app_template(
        &self,
        req: Request<DeleteAppTemplateReq>,
    ) -> Result<tonic::Response<DeleteAppTemplateResp>, Status> {
        let data = DeleteAppTemplateResp {
            code: todo!(),
            msg: todo!(),
        };
        Ok(Response::new(data))
    }

    async fn batch_delete_app_template(
        &self,
        req: Request<BatchDeleteAppTemplateReq>,
    ) -> Result<tonic::Response<BatchDeleteAppTemplateResp>, Status> {
        let data = BatchDeleteAppTemplateResp {
            code: todo!(),
            msg: todo!(),
        };
        Ok(Response::new(data))
    }
}
