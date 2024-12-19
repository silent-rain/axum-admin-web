use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

mod asset;
mod config;
mod router;

use config::AppConfig;
use database::PoolTrait;
use inject::InjectProvider;

use axum::{
    extract::Request,
    http::StatusCode,
    routing::{get, get_service},
    Extension, Router,
};
use colored::Colorize;
use dotenv::dotenv;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, warn};

/// 程序入口
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // 读取配置环境变量
    dotenv().ok();

    // 加载配置文件
    let app_config = AppConfig::new("config.yaml")?;

    // 初始化日志
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .with_line_number(true)
        .init();

    // 初始化数据库
    let db_pool = database::Pool::new(
        app_config.postgresql.dns(),
        app_config.postgresql.options.clone(),
    )
    .await?;

    // Using an Arc to share the provider across multiple threads.
    let inject_provider = Arc::new(InjectProvider::new(Arc::new(db_pool.clone())));

    // Build our application by creating our router.
    let app = Router::new()
        .nest("/api/v1", router::register()) // API 服务
        .nest_service("/upload", ServeDir::new("upload")) // 文件服务器, 指定到具体文件才可进行访问
        .nest_service("/static", ServeFile::new("static/index.html")) // 静态文件服务器
        .fallback(router::fallback) // 用于处理与路由器路由不匹配的任何请求
        .layer(Extension(app_config)) // 全局配置文件
        .layer(Extension(inject_provider)); // 依赖注入

    // Run our application as a hyper server
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0)? {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true)?;
            TcpListener::from_std(listener)?
        }
        // otherwise fall back to local listening
        None => {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
            TcpListener::bind(addr).await?
        }
    };

    info!(
        "listening on {}",
        listener.local_addr()?.to_string().yellow()
    );
    // Run the server with graceful shutdown
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(router::shutdown_signal())
        .await?;

    // 关闭数据库
    let _ = db_pool.close().await;
    info!("close database...");

    warn!("{}", "See you again~".yellow());
    Ok(())
}
