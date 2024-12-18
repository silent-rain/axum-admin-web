use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod router;

use anyhow::Ok;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .with_line_number(true)
        .init();

    // Build our application by creating our router.
    let app = Router::new()
        .fallback(router::fallback)
        .nest("/v1", router::register())
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
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Run the server with graceful shutdown
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(router::shutdown_signal())
        .await?;

    Ok(())
}
