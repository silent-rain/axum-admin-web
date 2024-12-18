//! 程序入口

use std::sync::Arc;

mod asset;
mod config;
mod router;
mod server;

use crate::asset::{AssetAdminWebDist, AssetConfigFile, AssetDbDataFile};

use app_state::{AppState, AssetState};
use database::PoolTrait;
use service_hub::inject::InjectProvider;
use timer::{TimerRegister, TimerShutdown};

use colored::Colorize;
use dotenv::dotenv;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// 程序入口
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    let _guards = logger::Logger::build(&conf.logger).expect("初始化日志失败");

    TimerRegister::start(db_pool.clone());

    // 共享状态
    let app_state = AppState {};
    let asset_state = Arc::new(AssetState {
        admin_web_dist: RwLock::new(Box::new(AssetAdminWebDist)),
        config_file: RwLock::new(Box::new(AssetConfigFile)),
        db_data_file: RwLock::new(Box::new(AssetDbDataFile)),
    });

    // 启动服务, 并阻塞
    if let Err(e) = server::start(app_state, asset_state, provider, conf).await {
        panic!("server start faild. err: {e}");
    }
    info!("close service...");

    TimerShutdown::shutdown().await.expect("关闭定时任务失败");
}
