#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("from utf8 error, {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("未找到资源")]
    AssetNotFound,
    #[error("资源解析错误")]
    AssetParseError,
}
