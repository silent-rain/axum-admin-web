//! 业务码
use std::io;

use serde::{Serialize, ser::Serializer};

/// 错误种类
#[derive(Debug, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    /// io error, no data available
    #[error("io error, no data available")]
    NoDataAvailable,
    /// io error, from io::Error
    #[error("io error, {0}")]
    Io(io::Error),
    /// from utf8 error, from std::string::FromUtf8Error
    #[error("from utf8 error, {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error("从JSON文本字符串中反序列化错误, {0}")]
    JsonDeserialization(String),

    #[error("结构序列化为JSON字符串错误, {0}")]
    JsonSerialization(String),

    #[error("parse file extension failed, {0}")]
    ParseFileExtension(String),
    #[error("convert type failed, {0}")]
    ConvertType(String),

    #[error("User-Agent解析错误, {0}")]
    UserAgentParserError(String),
    #[error("Uuid解析失败, {0}")]
    UuidParseError(String),
    #[error("调度任务移除解析失败, {0}")]
    ScheduleRemoveError(String),
    #[error("Get Schedule Instance Error")]
    ScheduleInstance,

    #[error("未找到资源")]
    AssetNotFound = 10290,
    #[error("资源解析错误")]
    AssetParseError = 10291,
    #[error("缓存不存在")]
    CacheNotFound = 10292,
    #[error("Casbin 策略执行失败, {0}")]
    CasbinEnforceError(String),
    #[error("No access permission")]
    CasbinNoAccessPermission,
}

/// 业务码序列化
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// IO 错误转换
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        if err.kind() == io::ErrorKind::UnexpectedEof {
            return Error::NoDataAvailable;
        }
        Error::Io(err)
    }
}
