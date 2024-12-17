//! 字典维度管理
use std::sync::Arc;

use crate::dto::dict_dimension::GetDictDimensionListReq;

use database::{Pagination, PoolTrait};
use entity::system::{sys_dict_dimension, SysDictDimension};
use nject::injectable;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

/// 数据访问
#[injectable]
pub struct DictDimensionDao {
    db: Arc<dyn PoolTrait>,
}

impl DictDimensionDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<sys_dict_dimension::Model>, u64), DbErr> {
        let results = SysDictDimension::find()
            .order_by_asc(sys_dict_dimension::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetDictDimensionListReq,
    ) -> Result<(Vec<sys_dict_dimension::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = SysDictDimension::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(sys_dict_dimension::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(sys_dict_dimension::Column::CreatedAt.lt(v))
            })
            .apply_if(req.name, |query, v| {
                query.filter(sys_dict_dimension::Column::Name.like(format!("{v}%")))
            })
            .apply_if(req.code, |query, v| {
                query.filter(sys_dict_dimension::Column::Code.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(sys_dict_dimension::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<sys_dict_dimension::Model>, DbErr> {
        SysDictDimension::find_by_id(id).one(self.db.db()).await
    }

    /// 通过名称获取详情信息
    pub async fn info_by_name(
        &self,
        name: String,
    ) -> Result<Option<sys_dict_dimension::Model>, DbErr> {
        SysDictDimension::find()
            .filter(sys_dict_dimension::Column::Name.eq(name))
            .one(self.db.db())
            .await
    }

    /// 通过编码获取详情信息
    pub async fn info_by_code(
        &self,
        code: String,
    ) -> Result<Option<sys_dict_dimension::Model>, DbErr> {
        SysDictDimension::find()
            .filter(sys_dict_dimension::Column::Code.eq(code))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn add(
        &self,
        active_model: sys_dict_dimension::ActiveModel,
    ) -> Result<sys_dict_dimension::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(
        &self,
        active_model: sys_dict_dimension::ActiveModel,
    ) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = SysDictDimension::update_many()
            .set(active_model)
            .filter(sys_dict_dimension::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn status(&self, id: i32, status: i8) -> Result<(), DbErr> {
        let active_model = sys_dict_dimension::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = SysDictDimension::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
