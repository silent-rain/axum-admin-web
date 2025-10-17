//! IntoErrorMsg 转换为 ErrorMsg

use crate::{Error, ErrorMsg};
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

pub trait IntoErrorMsg<T> {
    fn into_err(self) -> Result<T, ErrorMsg>;
    fn into_err_with_msg(self, msg: &str) -> Result<T, ErrorMsg>;
    fn into_err_with_appended_msg(self, msg: &str) -> Result<T, ErrorMsg>;
}

impl<T> IntoErrorMsg<T> for Result<T, Error> {
    fn into_err(self) -> Result<T, ErrorMsg> {
        self.map_err(ErrorMsg::form_err)
    }

    fn into_err_with_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| ErrorMsg::form_err(e).with_msg(msg))
    }

    fn into_err_with_appended_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| ErrorMsg::form_err(e).with_append_msg(msg))
    }
}

impl<T> IntoErrorMsg<T> for Result<T, anyhow::Error> {
    fn into_err(self) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();
            let mut msg = Error::Ok.msg();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
                msg = e.msg();
            }

            ErrorMsg::form_err(Error::Any(e))
                .with_code(code)
                .with_msg(&msg)
        })
    }

    fn into_err_with_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
            }

            ErrorMsg::form_err(Error::Any(e))
                .with_code(code)
                .with_msg(msg)
        })
    }

    fn into_err_with_appended_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
            }

            ErrorMsg::form_err(Error::Any(e))
                .with_code(code)
                .with_append_msg(msg)
        })
    }
}

impl<T> IntoErrorMsg<T> for Result<T, color_eyre::Report> {
    fn into_err(self) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();
            let mut msg = Error::Ok.msg();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
                msg = e.msg();
            }

            ErrorMsg::form_err(Error::ColorEyreReport(e))
                .with_code(code)
                .with_msg(&msg)
        })
    }

    fn into_err_with_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
            }

            ErrorMsg::form_err(Error::ColorEyreReport(e))
                .with_code(code)
                .with_msg(msg)
        })
    }

    fn into_err_with_appended_msg(self, msg: &str) -> Result<T, ErrorMsg> {
        self.map_err(|e| {
            let mut code = Error::Ok.code();

            if let Some(e) = e.downcast_ref::<Error>() {
                code = e.code();
            }

            ErrorMsg::form_err(Error::ColorEyreReport(e))
                .with_code(code)
                .with_append_msg(msg)
        })
    }
}
