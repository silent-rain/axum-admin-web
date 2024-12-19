use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

mod asset;
mod config;
mod router;

use config::AppConfig;
use inject::InjectProvider;

use anyhow::Ok;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use colored::Colorize;
use database::PoolTrait;
use dotenv::dotenv;
use listenfd::ListenFd;
use tokio::net::TcpListener;
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
        .fallback(router::fallback)
        .layer(Extension(app_config))
        .layer(Extension(inject_provider))
        .nest("/api/v1", router::register())
        .route("/", get(router::hello))
        .route("/hello/:name", get(router::json_hello))
        .route("/user", post(router::create_user));
    // // 静态生成的文件
    // .nest_service("/static", ServeDir::new("static"))
    // .nest_service(
    //     "/static",
    //     ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
    // );
    // .layer(Extension(config))
    // .layer(Extension(pool.clone()))
    // 构建一个使用此服务的后备服务
    // .fallback_service(get(|req| async move {
    //     match ServeDir::new(opt.static_dir).oneshot(req).await {
    //         Ok(res) => res.map(boxed),
    //         Err(err) => Response::builder()
    //             .status(StatusCode::INTERNAL_SERVER_ERROR)
    //             .body(boxed(Body::from(format!("error: {err}"))))
    //             .expect("error response"),
    //         }
    //     }
    // ))
    // .route(
    //     "/static",
    //     get_service(ServeFile::new("static/hello.html")).handle_error(
    //         |error: io::Error| async move {
    //             (
    //                 StatusCode::INTERNAL_SERVER_ERROR,
    //                 format!("Unhandled internal error: {}", error),
    //             )
    //         },
    //     ),
    // )

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
