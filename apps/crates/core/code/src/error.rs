// 业务码
use std::io;

use serde::{ser::Serializer, Serialize};

/// 错误种类
#[derive(Debug, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    /// ok
    #[error("ok")]
    OK = 0,
    /// unknown error
    #[error("unknown error, {0}")]
    Unknown(String) = 10001,
    /// internal server error
    #[error("internal server error, {0}")]
    InternalServer(String),
    /// request error
    #[error("request error, {0}")]
    RequestError(String),
    /// request timeout error
    #[error("request timeout, {0}")]
    RequestTimeout(String),
    /// invalid request parameter
    #[error("invalid request parameter, {0}")]
    InvalidParameter(String),
    #[error("Validate Errorr, {0}")]
    ValidateError(String) = 10106,

    /// config file parse error
    #[error("config file parse error, {0}")]
    ConfigFileParseError(String),

    #[error("Parse Request Body Error, {0}")]
    ParseRequestBodyError(String) = 10108,
    #[error("Expected request with `Content-Type: application/json`")]
    MissingJsonContentType,
    #[error("parse content-type error from header, {0}")]
    HeaderContentType(String),
    #[error("attempt to convert a string to a HeaderValue, {0}")]
    HeaderValue(String),

    // 数据处理异常
    /// Serialize the given data structure as a String of JSON.
    #[error("json serialization error, {0}")]
    JsonSerialization(String) = 10151,
    /// Deserialize an instance of type T from a string of JSON text.
    #[error("json deserialization error, {0}")]
    JsonDeserialization(String),
    #[error("json convert error, {0}")]
    JsonConvert(String),
    #[error("convert type failed, {0}")]
    ConvertType(String),
    /// io error, no data available
    #[error("io error, no data available")]
    NoDataAvailable,
    /// io error, from io::Error
    #[error("io error, {0}")]
    Io(io::Error),
    /// from utf8 error, from std::string::FromUtf8Error
    #[error("from utf8 error, {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("date time parse error, {0}")]
    DateTimeParseError(String),

    // 数据库操作
    #[error("db initialized error, {0}")]
    DbInit(String) = 10201,
    #[error("db not initialized")]
    DbNotInit,
    #[error("查询数据失败")]
    DbQueryError,
    #[error("未查到数据")]
    DbQueryEmptyError,
    #[error("添加数据失败")]
    DbAddError,
    #[error("批量添加数据失败")]
    DbBatchAddError,
    #[error("更新数据失败")]
    DbUpdateError,
    #[error("删除数据失败")]
    DbDeleteError,
    #[error("批量删除数据失败")]
    DbBatchDeleteError,
    #[error("更新数据状态失败")]
    DbUpdateStatusError,
    #[error("数据已存在")]
    DbDataExistError,
    #[error("数据已存在子项")]
    DbDataExistChildrenError,
    #[error("db table migration error, {0}")]
    DbTableMigration(String),

    // 验证码
    #[error("未知的验证码")]
    CaptchaNotExist = 10251,
    #[error("验证码已过期, 请刷新重试")]
    CaptchaExpire,
    #[error("验证码错误")]
    CaptchaInvalid,

    #[error("base64 decode error, {0}")]
    Base64Decode(String),

    // 鉴权
    #[error("账号或密码错误")]
    LoginPasswordError,
    #[error("用户已被禁用")]
    LoginUserDisableError,

    // JWT
    #[error("获取密匙异常")]
    TokenEncode,
    #[error("鉴权解析失败, err: {0}")]
    TokenDecode(String),
    #[error("获取鉴权标识失败")]
    HeadersNotAuthorization,
    #[error("获取鉴权前缀失败")]
    HeadersNotAuthorizationBearer,
    #[error("获取inject provider实例失败")]
    InjectAproviderObj,
    #[error("当前登陆态已失效, 请重新登陆")]
    LoginStatusDisabled,
    #[error("用户添加失败")]
    UserAddError,
    #[error("获取鉴权标识失败")]
    HeadersNotAuthorizationPassphrase,
    #[error("Illegal Request")]
    AuthIllegalRequest,

    // SESSION
    #[error("session id not found")]
    SessionIdNotFound,
    #[error("session id insert error, {0}")]
    SessionIdInsertError(String),

    #[error("数据库初始化失败, 管理员已存在无需重复初始化")]
    DbInitByAdminExistError,

    // 工具箱
    #[error("User-Agent解析错误, {0}")]
    UserAgentParserError(String) = 10381,
    #[error("Uuid解析失败, {0}")]
    UuidParseError(String),
    #[error("调度任务移除解析失败, {0}")]
    ScheduleRemoveError(String),
    #[error("Get Schedule Instance Error")]
    ScheduleInstance = 10284,

    #[error("缓存不存在")]
    CacheNotFound = 10292,
    #[error("Casbin 策略执行失败, {0}")]
    CasbinEnforceError(String),
    #[error("No access permission")]
    CasbinNoAccessPermission,

    // 文件或目录操作
    #[error("parse file extension failed, {0}")]
    ParseFileExtension(String) = 10501,
    #[error("获取目录失败")]
    FsReadDirError,
    #[error("获取上级目录失败")]
    FsParentDirError,
    #[error("创建目录失败")]
    FsCreateDir,
    #[error("读取文件失败, {0}")]
    FsReadFileError(String),
    #[error("创建文件失败, {0}")]
    FsCreateFileError(String),
    #[error("写入文件失败, {0}")]
    FsWriterFileError(String),
    #[error("parse file extension failed, {0}")]
    EmbedAssetError(String),

    // 业务逻辑
    #[error("Upload File Error, {0}")]
    UploadFileError(String) = 20001,
    #[error("failed to generate user sharing code")]
    GenerateUserShareCore,

    /// 自定义错误
    #[error("自定义错误")]
    CustomError = 65535,
    // Other error from higher-level crate, for downcasting
    // Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl Error {
    /// 返回错误码
    pub fn code(&self) -> u16 {
        unsafe {
            let ptr = self as *const Error as *const u16;
            ptr.read_volatile()
        }
    }
    /// 返回错误码信息
    pub fn msg(&self) -> String {
        self.to_string()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code() {
        let mut err = Error::Unknown("0".to_string());
        assert!(err.to_string() == "unknown error, 0");

        let code = unsafe {
            let mul_err = &mut err;
            let ptr: *const u16 = mul_err as *mut Error as *const u16;
            ptr.read_volatile()
        };
        assert!(code == 10001);
    }

    #[test]
    fn test_error_code2() {
        let err = Error::Unknown("0".to_string());
        let code = err.code();
        assert!(code == 10001);
    }
}
