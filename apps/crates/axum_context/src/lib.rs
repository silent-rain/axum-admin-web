//! 上下文管理
//! extractors: https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
mod context;
pub use context::{ApiAuthType, Context};

mod layer;
pub use layer::ContextLayer;

pub mod extractor;
