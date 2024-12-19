//! 配置文件
#![allow(unused)]

use std::fs::read_to_string;
use std::sync::OnceLock;

pub mod env;
pub mod server;

use code::Error;
use logger::config::LoggerConfig;

use serde::{Deserialize, Serialize};
use tracing::error;

/// 全局配置对象
static GLOBAL_CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// 初始化, 解析配置文件
/// # Examples
///
/// ```
/// let config = init("./config.yaml");
/// assert!(config.is_ok());
/// ```
pub fn init(path: &str) -> Result<AppConfig, Error> {
    let content = read_to_string(path)?;
    let config: AppConfig = serde_yaml::from_str(&content).map_err(|err| {
        error!(
            "{}, err: {err}",
            Error::ConfigFileParseError(err.to_string())
        );
        Error::ConfigFileParseError(err.to_string())
    })?;
    GLOBAL_CONFIG.get_or_init(|| config.clone());
    Ok(config)
}

/// 获取全局配置
/// # Examples
/// ```
/// config = instance()
/// assert!(config.is_ok());
/// ```
pub fn instance() -> &'static AppConfig {
    let config = GLOBAL_CONFIG.get();
    match config {
        Some(config) => config,
        None => {
            panic!("configuration not initialized!")
        }
    }
}

/// 全局配置 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    /// 环境配置
    #[serde(default)]
    pub env: env::Env,
    /// 服务配置
    #[serde(default)]
    pub server: server::Server,
    /// MySQL 数据库配置
    #[serde(default)]
    pub mysql: database::Config,
    /// PostgreSQL 数据库配置
    #[serde(default)]
    pub postgresql: database::Config,
    /// Sqlite3 数据库配置
    #[serde(default)]
    pub sqlite: database::Config,
    /// 日志配置
    #[serde(default)]
    pub logger: LoggerConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let path = "config.yaml";
        let config = init(path);
        println!("config:\n{:#?}", config);
        assert!(config.is_ok())
    }

    #[test]
    fn test_include_str() {
        let yaml_str = include_str!("../../config.yaml");
        println!("config:\n{:#?}", yaml_str);
        assert_ne!(yaml_str, "");
    }
}
