//! 图片验证码管理
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::system::image_captcha;

/// 获取验证码列表
#[derive(Default, Deserialize, Validate)]
pub struct GetImageCaptchasReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetImageCaptchasResp {
    pub data_list: Vec<image_captcha::Model>,
    pub total: u64,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetImageCaptchaReq {
    /// 验证码ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetImageCaptchaResp {
    #[serde(flatten)]
    data: image_captcha::Model,
}

/// 查询数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetInfoByCaptchaIdReq {
    /// 验证码ID
    pub captcha_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInfoByCaptchaIdResp {
    #[serde(flatten)]
    data: image_captcha::Model,
}

/// 添加数据 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct CreateImageCaptchaReq {}

/// 添加验证码 响应体
#[derive(Default, Deserialize, Serialize)]
pub struct CreateImageCaptchaResp {
    /// 验证码ID
    pub captcha_id: String,
    /// 图片数据, Base64编码
    pub data: String,
    /// 创建时间
    pub created_at: NaiveDateTime,
}

/// 删除数据 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteImageCaptchaReq {
    /// 验证码ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteImageCaptchaResp {}

/// 批量删除验证码
#[derive(Default, Deserialize, Validate)]
pub struct BatchDeleteImageCaptchaReq {
    /// ID列表
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchDeleteImageCaptchaResp {}

/// 获取图片 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct ShowCaptchaImageReq {
    /// 验证码ID
    pub captcha_id: String,
}
