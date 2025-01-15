//! 图片验证码管理

use crate::{
    dto::image_captcha::{
        BatchDeleteImageCaptchaReq, BatchDeleteImageCaptchaResp, CreateImageCaptchaReq,
        CreateImageCaptchaResp, DeleteImageCaptchaReq, DeleteImageCaptchaResp, GetImageCaptchaReq,
        GetImageCaptchaResp, GetImageCaptchasReq, GetImageCaptchasResp, GetInfoByCaptchaIdReq,
        GetInfoByCaptchaIdResp,
    },
    service::image_captcha::ImageCaptchaService,
};

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};
use inject::AInjectProvider;

/// 控制器
pub struct ImageCaptchaController;

impl ImageCaptchaController {
    /// 获取验证码列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetImageCaptchasReq>,
    ) -> Responder<GetImageCaptchasResp> {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let (results, total) = image_captcha_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取验证码信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetImageCaptchaReq>,
    ) -> Responder<GetImageCaptchaResp> {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let result = image_captcha_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取验证码信息
    pub async fn info_by_captcha_id(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetInfoByCaptchaIdReq>,
    ) -> Responder<GetInfoByCaptchaIdResp> {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let result = image_captcha_service.info_by_captcha_id(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加验证码
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<CreateImageCaptchaReq>,
    ) -> Responder<CreateImageCaptchaResp> {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let result = image_captcha_service.create(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 删除验证码
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteImageCaptchaReq>,
    ) -> Responder<DeleteImageCaptchaResp> {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let _result = image_captcha_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 批量删除验证码
    pub async fn batch_delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<BatchDeleteImageCaptchaReq>,
    ) -> Responder<BatchDeleteImageCaptchaResp> {
        let image_captcha_service: ImageCaptchaService = provider.provide();
        let _result = image_captcha_service.batch_delete(req.ids.clone()).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[test]
    fn test_uuid() {
        let uuid = Uuid::new_v4().to_string();
        assert_eq!(uuid.len(), 36);
    }
}
