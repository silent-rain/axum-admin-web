//! 数据库注入

use std::sync::Arc;

use nject::injectable;

use database::PoolTrait;

#[injectable]
pub struct Mdb {
    /// 定义主业务数据库类型
    pub main_db: Arc<dyn PoolTrait>,
    /// 定义配置数据库类型
    pub config_db: Arc<dyn PoolTrait>,
}

impl Mdb {
    pub fn new(main_db: Arc<dyn PoolTrait>, config_db: Arc<dyn PoolTrait>) -> Self {
        Mdb { main_db, config_db }
    }
}
