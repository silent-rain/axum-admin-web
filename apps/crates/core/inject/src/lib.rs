//! 依赖注入
use std::sync::Arc;

use database::PoolTrait;

use nject::provider;

pub mod mdb;
use mdb::Mdb;

#[provider]
pub struct InjectProvider {
    #[provide(Arc<dyn PoolTrait>, |x| x.clone())]
    db: Arc<dyn PoolTrait>,
    #[provide]
    mdb: Mdb,
}

impl InjectProvider {
    pub fn new(main_db: Arc<dyn PoolTrait>, config_db: Arc<dyn PoolTrait>) -> Self {
        let mdb = Mdb::new(main_db.clone(), config_db);
        InjectProvider { db: main_db, mdb }
    }
}

pub type AInjectProvider = Arc<InjectProvider>;

// 实现自定义 Injectable trait
// impl<'a> Injectable<'a, Arc<dyn PoolTrait>, InjectProvider> for Arc<dyn PoolTrait> {
//     fn inject(provider: &'a InjectProvider) -> Self {
//         provider.db.clone()
//     }
// }
