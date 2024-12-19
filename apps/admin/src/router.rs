//! 路由

use std::collections::HashMap;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use response::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::signal;

use middleware::cors::cors_layer;

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// 优雅关机
pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

pub async fn hello() -> String {
    "Hello, World!".into()
}

pub async fn hello2() -> Response<()> {
    Response::<()>::ok()
}

// pub fn hello31() -> Responder<String> {
//     let x = Some("1".to_string()).ok_or_else(|| Error::Unknown("xxx".to_owned()))?;
//     Ok(Response::data(x))
// }

pub async fn hello3() -> impl IntoResponse {
    Response::<()>::ok()
}

pub async fn hello4() -> impl IntoResponse {
    Response::data("1")
}

pub async fn hello5() -> impl IntoResponse {
    let data_list = vec![1, 2, 3, 4, 5];
    let total = data_list.len() as u64;
    Response::data_list(data_list, total)
}

/// 从路径中提取参数
pub async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
    let greeting = name.as_str();
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({"message": hello + greeting })))
}

/// axum handler for "GET /demo-query" which uses `axum::extract::Query`.
/// This extracts query parameters and creates a key-value pair map.
pub fn get_demo_query(
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> String {
    format!("Demo query params: {:?}", params)
}

// /// axum handler for "POST /demo-form" which submits an HTML form.
// /// This demo shows how extract a form submission to a struct.
// pub async fn post_demo_form(form: axum::extract::Form<Book>) -> axum::response::Html<String> {
//     let book: Book = form.0;
//     format!(
//         r#"
//         <!doctype html>
//         <html>
//             <head>
//                 <title>Book</title>
//             </head>
//             <body>
//                 <h1>Book</h1>
//                 {:?}
//             </body>
//         </html>
//         "#,
//         &book
//     )
//     .into()
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct User {
    id: u64,
    username: String,
}

pub async fn create_user(Json(payload): Json<User>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

/// 路由器
pub struct LocationRouter;

impl LocationRouter {
    /// 注册`用户地理位置管理`路由
    pub fn register() -> Router {
        let routes = Router::new()
            .route("/hello2", get(hello2))
            .route("/hello3", get(hello3))
            .route("/hello4", get(hello4))
            .route("/hello5", get(hello5))
            .route("/x", get(hello))
            .route("/x/:id", get(hello).delete(hello).put(hello))
            .route("/add", post(hello))
            .route("/status", put(hello));

        Router::new().nest("/locations", routes)
    }
}

/// 全局应用状态
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AppState {}

// go ahead and run "cargo run main.rs"
// localhost:4000 should now print out your user agent
// async fn index(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
//     String::from(user_agent.as_str())
// }

async fn check_cookie(jar: PrivateCookieJar) -> impl IntoResponse {
    if let None = jar.get("hello") {
        jar.add(Cookie::new("hello", "world"));
        // jar.remove(Cookie::from("foo"));
    }

    StatusCode::OK
}

/// 注册路由
pub fn register() -> Router {
    // let state = AppState {};

    Router::new()
        // >>> 中间件 >>>
        // 注意中间件加载顺序: Last in, first loading
        // .wrap(ApiOperation::default())
        // .wrap(TracingLogger::default())
        .layer(cors_layer())
        // // 接口鉴权
        // .wrap(CasbinAuth::default())
        // .wrap(SystemApiAuth::default())
        // .wrap(OpenApiAuth::default())
        // // 上下文中间件
        // .wrap(ContextMiddleware::default())
        // <<< 中间件 <<<
        // .merge( HealthRouter::register()) // 健康检查
        .nest("/v1", LocationRouter::register())
}
