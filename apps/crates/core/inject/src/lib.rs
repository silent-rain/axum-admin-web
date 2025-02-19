//! 依赖注入
use std::sync::Arc;

use database::{Mdb, PoolTrait};

use nject::provider;

#[provider]
pub struct InjectProvider {
    #[provide(Arc<dyn PoolTrait>, |x| x.clone())]
    db: Arc<dyn PoolTrait>,
    #[provide]
    mdb: Mdb,
}

impl InjectProvider {
    pub fn new(db_pool: Mdb) -> Self {
        InjectProvider {
            db: db_pool.main_db.clone(),
            mdb: db_pool,
        }
    }
}

pub type AInjectProvider = Arc<InjectProvider>;

// 实现自定义 Injectable trait
// impl<'a> Injectable<'a, Arc<dyn PoolTrait>, InjectProvider> for Arc<dyn PoolTrait> {
//     fn inject(provider: &'a InjectProvider) -> Self {
//         provider.db.clone()
//     }
// }
