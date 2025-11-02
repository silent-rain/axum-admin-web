//! 任务调度事件日志表
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 定时任务事件状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum Status {
    /// 开始
    Start = 0,
    /// 完成
    Done = 1,
    /// 停止
    Stop = 2,
    /// 移除
    Removed = 3,
}
