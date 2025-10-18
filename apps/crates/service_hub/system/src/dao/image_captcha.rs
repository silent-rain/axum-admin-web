//! 图片验证码管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

use database::{Pagination, PoolTrait};
use entity::system::{ImageCaptchaEntity, image_captcha};

use crate::dto::image_captcha::GetImageCaptchasReq;

/// 数据访问
#[injectable]
pub struct ImageCaptchaDao {
    db: Arc<dyn PoolTrait>,
}

impl ImageCaptchaDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetImageCaptchasReq,
    ) -> Result<(Vec<image_captcha::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = ImageCaptchaEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(image_captcha::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(image_captcha::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(image_captcha::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }
    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<image_captcha::Model>, DbErr> {
        ImageCaptchaEntity::find()
            .filter(image_captcha::Column::Id.eq(id))
            .one(self.db.db())
            .await
    }

    /// 通过captcha_id获取详情信息
    pub async fn info_by_captcha_id(
        &self,
        captcha_id: String,
    ) -> Result<Option<image_captcha::Model>, DbErr> {
        ImageCaptchaEntity::find()
            .filter(image_captcha::Column::CaptchaId.eq(captcha_id))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: image_captcha::ActiveModel,
    ) -> Result<image_captcha::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新信息
    pub async fn update(&self, data: image_captcha::Model) -> Result<u64, DbErr> {
        // Into ActiveModel
        let pear: image_captcha::ActiveModel = data.clone().into();

        let result = ImageCaptchaEntity::update_many()
            .set(pear)
            .filter(image_captcha::Column::Id.eq(data.id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, status: bool) -> Result<(), DbErr> {
        let active_model = image_captcha::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = ImageCaptchaEntity::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 按主键批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = ImageCaptchaEntity::delete_many()
            .filter(image_captcha::Column::Id.is_in(ids))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
