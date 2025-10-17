use axum::{
    body::Body,
    http::{Response, StatusCode},
};
use log::error;
use serde::{Serialize, Serializer};
use serde_json::json;

#[derive(Debug, PartialEq, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    #[error("internal server error")]
    InternalServer,

    #[error("illegal request")]
    HeadersNotAuthorization,
    #[error("illegal request")]
    HeadersNotAuthorizationBearer,

    /// 令牌已经过期
    #[error("Token has expired")]
    CheckExp,
    /// Audience 验证错误
    #[error("Audience verification failed")]
    CheckAud,
    /// Issued At 时间验证错误
    #[error("Issued At time verification failed")]
    CheckIat,
    /// Not Before 时间验证错误
    #[error("Not Before time verification failed")]
    CheckNbf,
    /// Subject 验证错误
    #[error("Subject verification failed")]
    CheckSub,
    /// 发行人验证错误
    #[error("Issuer verification failed")]
    CheckIss,
    /// JWT 错误
    #[error(transparent)]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
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

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorMsg {
    code: u16,
    msg: String,
    inner: Option<Error>,
}

impl std::fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorMsg(code: {}, msg: {})", self.code, self.msg)
    }
}

impl ErrorMsg {
    /// 从 Error 创建一个新的错误信息
    pub fn form_err(err: Error) -> Self {
        Self {
            code: err.code(),
            msg: err.msg(),
            inner: Some(err),
        }
    }

    /// 重置错误码
    pub fn with_code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    /// 重置错误信息
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    /// 追加错误信息, 在错误码信息的基础上添加新的信息
    pub fn with_append_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg);
        self
    }
}

impl Error {
    pub fn into_err(self) -> ErrorMsg {
        ErrorMsg::form_err(self)
    }

    pub fn into_err_with_msg(self, msg: &str) -> ErrorMsg {
        ErrorMsg::form_err(self).with_msg(msg)
    }

    pub fn into_err_with_appended_msg(self, msg: &str) -> ErrorMsg {
        ErrorMsg::form_err(self).with_append_msg(msg)
    }
}

impl From<Error> for ErrorMsg {
    fn from(err: Error) -> Self {
        ErrorMsg {
            code: err.code(),
            msg: err.msg(),
            inner: Some(err),
        }
    }
}

impl From<ErrorMsg> for Response<Body> {
    fn from(err_msg: ErrorMsg) -> Self {
        let data = serde_json::to_string(&err_msg).unwrap_or_else(|e| {
            error!("转换为JSON字符串失败, error: {:#?}", e);

            json!({
                "code": Error::InternalServer.code(),
                "msg": Error::InternalServer.msg()
            })
            .to_string()
        });

        let mut resp = Response::new(Body::from(data));
        *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

        resp
    }
}
