//! 上下文管理
//!
//! 中间件初始化时, 或从cookie中获取session_id, 如果存在则插入上下文中, 不存在则重新生成session_id并插入上下文中。
//! extractors: https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
mod error;

mod context;
pub use context::{ApiAuthType, Context};

mod layer;
pub use layer::ContextLayer;

pub mod extractor;
