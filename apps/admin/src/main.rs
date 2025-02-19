//! 程序入口
use std::sync::Arc;

mod asset;
mod config;
mod router;
mod server;

use app_state::AppState;
use config::AppConfig;
use database::Mdb;
use inject::InjectProvider;
use server::HttpServer;

use colored::Colorize;
use dotenv::dotenv;
use tracing::warn;

/// 程序入口
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // 读取配置环境变量
    dotenv().ok();

    // 加载配置文件
    let app_config = AppConfig::new("config.yaml")?;

    // 初始化日志
    let _guards = logger::Logger::build(&app_config.logger).expect("初始化日志失败");

    // 初始化数据库
    let main_db =
        database::Pool::new(app_config.mysql.dns(), app_config.mysql.options.clone()).await?;
    let db_pool = Mdb::new(Arc::new(main_db.clone()), Arc::new(main_db));

    // 全局状态
    let state = Arc::new(AppState {});

    // Using an Arc to share the provider across multiple threads.
    let inject_provider = Arc::new(InjectProvider::new(db_pool.clone()));

    // 定时任务
    // TimerRegister::start(db_pool.clone());

    // 阻塞运行服务
    HttpServer::run(app_config, db_pool.clone(), inject_provider, state).await?;

    // 关闭数据库
    let _ = db_pool.close().await;

    warn!("{}", "See you again~".yellow());
    Ok(())
}
