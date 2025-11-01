//! 数据库注入

use std::sync::Arc;

use colored::Colorize;
use log::{error, info};

use crate::PoolTrait;

#[derive(Clone)]
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

    /// 关闭数据库实例
    pub async fn close(&self) {
        if let Err(e) = self.main_db.close().await {
            error!("Failed to close main database: {}", e);
        }
        if let Err(e) = self.config_db.close().await {
            error!("Failed to close config database: {}", e);
        }

        info!("{}", "The database has been closed".yellow());
    }
}

// impl Drop for Mdb {
//     fn drop(&mut self) {
//         let main_db = self.main_db.clone();
//         let config_db = self.config_db.clone();
//         tokio::spawn(async move {
//             if let Err(e) = main_db.close().await {
//                 error!("Failed to close main database: {}", e);
//             }
//             if let Err(e) = config_db.close().await {
//                 error!("Failed to close config database: {}", e);
//             }
//             info!("{}", "The database has been closed".yellow());
//         });
//     }
// }
