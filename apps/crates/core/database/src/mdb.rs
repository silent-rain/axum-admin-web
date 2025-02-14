//! 数据库集合

use std::sync::Arc;

use sea_orm::DbErr;

use crate::PoolTrait;

pub struct Mdb {
    /// 默认主数据库
    pub db: Arc<dyn PoolTrait>,
    /// App 配置数据库
    pub app_db: Arc<dyn PoolTrait>,
}
