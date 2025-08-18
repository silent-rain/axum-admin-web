//! 任务调度作业表

use serde_repr::{Deserialize_repr, Serialize_repr};

/// 任务调度状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum Status {
    /// 下线
    Offline = 0,
    /// 上线
    Online = 1,
}

/// 定时任务类型
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum JobType {
    /// 任务调度
    Timer = 0,
    /// 即时任务
    Interval = 1,
}

/// 定时任务来源
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum Source {
    /// 用户定义
    User = 0,
    /// 系统内部
    System = 1,
}
