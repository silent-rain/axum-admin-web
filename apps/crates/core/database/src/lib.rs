//! 数据库

mod curd;
pub use curd::Curd;

mod pagination;
pub use pagination::Pagination;

mod pool;
pub use pool::{Pool, PoolTrait};
pub use sea_orm::DatabaseConnection;

pub mod config;
pub use config::DbOptions;

pub mod mock;
