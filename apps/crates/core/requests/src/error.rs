//! 错误类型

use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};

/// 错误类型
#[derive(Debug, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    #[error("Invalid Header Name, {0}")]
    InvalidHeaderName(#[from] InvalidHeaderName),
    #[error("Invalid Header Value, {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("Reqwest Error, {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Io Error, {0}")]
    IoError(#[from] std::io::Error),

    #[error("File Name Extension Error, {0}")]
    FileNameExtension(String),

    #[error("Type Convert Error, {0}")]
    ConvertType(String),
}
