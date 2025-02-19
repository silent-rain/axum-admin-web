//! 服务
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use app_state::AppState;
use database::Mdb;
use inject::InjectProvider;
use service_hub::public::AdminWebSiteRouter;

use axum::{Extension, Router};
use colored::Colorize;
use listenfd::ListenFd;
use tokio::net::TcpListener;
use tracing::info;

use crate::{config::AppConfig, router};

/// Http 服务
pub struct HttpServer {}

impl HttpServer {
    /// 运行服务
    pub async fn run(
        app_config: AppConfig,
        db_pool: Mdb,
        inject_provider: Arc<InjectProvider>,
        state: Arc<AppState>,
    ) -> anyhow::Result<()> {
        // Build our application by creating our router.
        let app = Router::new()
            .nest("/api/v1", router::register(db_pool.main_db.clone())) // API 服务
            .merge(AdminWebSiteRouter::register()) // 后台WEB服务
            // .nest_service("/", ServeFile::new("../../admin-web/dist/index.html")) // 静态文件服务器
            // .nest("/", axum_static::static_router("../../admin-web/dist/"))
            // .nest_service(
            //     "/wb/*key",
            //     ServeDir::new("../../admin-web/dist").append_index_html_on_directories(true),
            // ) // 文件服务器, 指定到具体文件才可进行访问
            .fallback(router::fallback) // 用于处理与路由器路由不匹配的任何请求
            .layer(Extension(app_config)) // 全局配置文件
            .layer(Extension(inject_provider)) // 依赖注入
            .layer(Extension(state)); // 全局状态

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
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(router::shutdown_signal())
        .await?;

        Ok(())
    }
}
