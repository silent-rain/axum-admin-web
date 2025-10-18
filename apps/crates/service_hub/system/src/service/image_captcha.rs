//! 图片验证码管理

use base64::{Engine, engine::general_purpose};
use log::{error, warn};
use nject::injectable;
use sea_orm::Set;
use uuid::Uuid;

use axum_response::ResponseErr;
use entity::system::image_captcha;
use err_code::Error;
use utils::captcha::generate_captcha;

use crate::{
    constant::CAPTCHA_EXPIRE,
    dao::image_captcha::ImageCaptchaDao,
    dto::image_captcha::{
        CreateImageCaptchaReq, CreateImageCaptchaResp, DeleteImageCaptchaReq, GetImageCaptchaReq,
        GetImageCaptchasReq, GetInfoByCaptchaIdReq, ShowCaptchaImageReq,
    },
};

/// 服务层
#[injectable]
pub struct ImageCaptchaService {
    image_captcha_dao: ImageCaptchaDao,
}

impl ImageCaptchaService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetImageCaptchasReq,
    ) -> Result<(Vec<image_captcha::Model>, u64), ResponseErr> {
        let (results, total) = self.image_captcha_dao.list(req).await.map_err(|err| {
            error!("查询验证码列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询验证码列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetImageCaptchaReq) -> Result<image_captcha::Model, ResponseErr> {
        let result = self
            .image_captcha_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询验证码信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询验证码信息失败")
            })?
            .ok_or_else(|| {
                error!("验证码不存在");
                Error::DbQueryEmptyError.into_err_with_msg("验证码不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        _req: CreateImageCaptchaReq,
    ) -> Result<CreateImageCaptchaResp, ResponseErr> {
        // 生成验证码
        let (captcha, base_img) = generate_captcha();
        let captcha_id = Uuid::new_v4().to_string();
        let expire = CAPTCHA_EXPIRE;

        let img_bytes = general_purpose::STANDARD
            .decode(base_img.clone().as_bytes())
            .map_err(|err| {
                error!("base64 decode error, err: {:#?}", err);
                Error::Base64Decode(err.to_string()).into_err()
            })?;

        let model = image_captcha::ActiveModel {
            captcha_id: Set(captcha_id),
            captcha: Set(captcha.clone()),
            data: Set(img_bytes),
            expire: Set(expire),
            ..Default::default()
        };
        let result = self.image_captcha_dao.create(model).await.map_err(|err| {
            error!("添加验证码信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加验证码信息失败")
        })?;

        let result = CreateImageCaptchaResp {
            captcha_id: result.captcha_id,
            data: format!("data:image/jpeg;base64,{}", base_img),
            created_at: result.created_at,
        };
        // TODO 后期调整日志级别
        warn!(
            "Generate verification code, captcha_id: {} captcha: {}",
            result.captcha_id, captcha
        );
        Ok(result)
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteImageCaptchaReq) -> Result<u64, ResponseErr> {
        let result = self.image_captcha_dao.delete(req.id).await.map_err(|err| {
            error!("删除验证码信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除验证码信息失败")
        })?;

        Ok(result)
    }

    /// 批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, ResponseErr> {
        let result = self
            .image_captcha_dao
            .batch_delete(ids)
            .await
            .map_err(|err| {
                error!("批量删除验证码信息失败, err: {:#?}", err);
                Error::DbBatchDeleteError.into_err_with_msg("批量删除验证码信息失败")
            })?;

        Ok(result)
    }
}

impl ImageCaptchaService {
    /// 通过captcha_id获取详情信息
    pub async fn info_by_captcha_id(
        &self,
        req: GetInfoByCaptchaIdReq,
    ) -> Result<image_captcha::Model, ResponseErr> {
        let result = self
            .image_captcha_dao
            .info_by_captcha_id(req.captcha_id)
            .await
            .map_err(|err| {
                error!("查询验证码信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询验证码信息失败")
            })?
            .ok_or_else(|| {
                error!("验证码不存在");
                Error::DbQueryEmptyError.into_err_with_msg("验证码不存在")
            })?;

        // 验证码在使用后将其状态更新为无效
        self.image_captcha_dao
            .update_status(result.id, false)
            .await
            .map_err(|err| {
                error!("更新验证码状态失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("更新验证码状态失败")
            })?;

        Ok(result)
    }

    /// 通过captcha_id获取详情信息
    pub async fn show_image(
        &self,
        req: ShowCaptchaImageReq,
    ) -> Result<image_captcha::Model, ResponseErr> {
        let result = self
            .image_captcha_dao
            .info_by_captcha_id(req.captcha_id)
            .await
            .map_err(|err| {
                error!("查询验证码信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询验证码信息失败")
            })?
            .ok_or_else(|| {
                error!("验证码不存在");
                Error::DbQueryEmptyError.into_err_with_msg("验证码不存在")
            })?;

        Ok(result)
    }
}
