//! 数据库集合

use crate::PoolTrait;

/// 定义主业务数据库类型
pub trait MainDB: PoolTrait {}

/// 定义配置数据库类型
pub trait ConfigDB: PoolTrait {}
