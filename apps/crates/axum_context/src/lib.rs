//! 上下文管理
//! extractors: https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
mod context;
pub use context::{ApiAuthType, Context};

mod middleware;
pub use middleware::ContextLayer;

pub mod extractor;
