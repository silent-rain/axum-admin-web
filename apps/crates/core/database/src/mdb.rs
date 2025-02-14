//! 数据库集合

use crate::PoolTrait;

pub trait DbPoolTrait: PoolTrait {}
pub trait AppDbPoolTrait: PoolTrait {}
