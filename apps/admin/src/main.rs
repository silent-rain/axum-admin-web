use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

mod asset;
mod config;
mod router;

use anyhow::Ok;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use colored::Colorize;
use database::PoolTrait;
use dotenv::dotenv;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, warn};

// use service_hub::inject::InjectProvider;

/// 程序入口
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // 读取配置环境变量
    dotenv().ok();

    // 加载配置文件
    let conf = config::init("config.yaml").expect("配置文件加载失败");

    // 初始化日志
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .with_line_number(true)
        .init();

    // mysql dns
    let database_url = conf.mysql.dns();
    // sqlite dns
    // let database_url = conf.sqlite.dns();

    // 初始化数据库
    let db_pool = database::Pool::new(database_url, conf.mysql.options.clone())
        .await
        .expect("初始化数据库失败");

    // Using an Arc to share the provider across multiple threads.
    // let provider = InjectProvider::new(Arc::new(db_pool.clone()));
    // let provider = Arc::new(provider);

    // Build our application by creating our router.
    let app = Router::new()
        .fallback(router::fallback)
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
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
    info!("listening on {}", addr.to_string().yellow());
    let listener = tokio::net::TcpListener::bind(addr).await?;

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
