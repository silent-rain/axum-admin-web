//! 数据库配置
use serde::{Deserialize, Serialize};

/// 数据库类型
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum DbType {
    #[default]
    #[serde(rename = "mysql")]
    Mysql,
    #[serde(rename = "postgresql")]
    PostgreSQL,
    #[serde(rename = "sqlite")]
    Sqlite,
}

/// Mysql 数据库配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// 数据库类型
    #[serde(default)]
    pub r#type: DbType,
    /// db信息唯一标识
    #[serde(default)]
    pub key: String,
    /// IP或域名
    #[serde(default)]
    pub host: String,
    /// 端口
    #[serde(default)]
    pub port: i32,
    /// 账号
    #[serde(default)]
    pub username: String,
    /// 密码
    #[serde(default)]
    pub password: String,
    /// 数据库名称
    #[serde(default)]
    pub db_name: String,
    /// Setting default PostgreSQL schema
    #[serde(default)]
    pub schema: String,
    /// sqlite 路径
    #[serde(default)]
    pub sqlite_path: Option<String>,
    /// 数据库参数
    #[serde(default)]
    pub options: Options,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            r#type: DbType::Mysql,
            key: "".to_string(),
            host: "".to_string(),
            port: 0,
            username: "".to_string(),
            password: "".to_string(),
            db_name: "".to_string(),
            schema: "public".to_string(),
            sqlite_path: None,
            options: Options::default(),
        }
    }
}

impl Config {
    /// 数据库地址
    /// 不支持时区
    pub fn dns(&self) -> String {
        match self.r#type {
            DbType::Mysql => {
                // 这些参数会导致连接失败: ?charset=utf8mb4&parseTime=false&loc=Asia%2FShanghai
                // loc=Local
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.db_name,
                )
            }
            DbType::PostgreSQL => {
                format!(
                    "postgres://{}:{}@{}:{}/{}?currentSchema={}",
                    self.username, self.password, self.host, self.port, self.db_name, self.schema,
                )
            }
            DbType::Sqlite => {
                // Read only: sqlite://path/to/db.sqlite?mode=ro
                // Create file if not exists: sqlite://path/to/db.sqlite?mode=rwc
                // In memory: sqlite::memory:
                self.sqlite_path
                    .clone()
                    .map_or_else(|| "data.dat?mode=rwc".to_string(), |v| v)
            }
        }
    }
}

/// 参数配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Options {
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

impl Default for Options {
    fn default() -> Self {
        Self {
            max_connections: 100,
            min_connections: 5,
            connect_timeout: 8,
            acquire_timeout: 8,
            idle_timeout: 8,
            max_lifetime: 8,
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
