//! 数据库配置
use serde::{Deserialize, Serialize};

/// Mysql 数据库配置
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MysqlConfig {
    /// db信息唯一标识
    pub key: String,
    /// IP或域名
    pub host: String,
    /// 端口
    pub port: i32,
    /// 账号
    pub username: String,
    /// 密码
    pub password: String,
    /// 数据库名称
    pub db_name: String,
    /// 数据库参数
    pub options: DbOptions,
}

impl MysqlConfig {
    /// 数据库地址
    pub fn dns(&self) -> String {
        // 这些参数会导致连接失败: ?charset=utf8mb4&parseTime=false&loc=Asia%2FShanghai
        // loc=Local
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name,
        )
    }
}

/// 参数配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbOptions {
    /// Set the maximum number of connections of the pool
    pub max_connections: u32,
    /// Set the minimum number of connections of the pool
    pub min_connections: u32,
    /// Set the timeout duration when acquiring a connection
    pub connect_timeout: u64,
    /// Set the maximum amount of time to spend waiting for acquiring a connection
    pub acquire_timeout: u64,
    /// Set the idle duration before closing a connection
    pub idle_timeout: u64,
    /// Set the maximum lifetime of individual connections
    pub max_lifetime: u64,
    /// Enable SQLx statement logging (default true)
    pub logging_enable: bool,
    /// Set SQLx statement logging level (default INFO). (ignored if sqlx_logging is false)
    pub logging_level: Level,
}

impl Default for DbOptions {
    fn default() -> Self {
        Self {
            max_connections: 8,
            min_connections: 5,
            connect_timeout: 10,
            acquire_timeout: 10,
            idle_timeout: 10,
            max_lifetime: 10,
            logging_enable: true,
            logging_level: Level::Info,
        }
    }
}

/// 日志级别
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Level {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

// String 别转换为 log::LevelFilter
impl From<Level> for log::LevelFilter {
    fn from(level: Level) -> Self {
        match level {
            Level::Off => log::LevelFilter::Off,
            Level::Trace => log::LevelFilter::Trace,
            Level::Debug => log::LevelFilter::Debug,
            Level::Info => log::LevelFilter::Info,
            Level::Warn => log::LevelFilter::Warn,
            Level::Error => log::LevelFilter::Error,
        }
    }
}
