//! 任务调度状态日志表
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 定时任务事件状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum Status {
    /// 运行中
    Running = 0,
    /// 完成
    Completed = 1,
    /// 失败
    Failed = 2,
}
