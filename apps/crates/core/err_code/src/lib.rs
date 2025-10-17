//! 业务码
mod error;
pub use error::Error;

mod error_msg;
pub use error_msg::ErrorMsg;

mod into_error_msg;
pub use into_error_msg::IntoErrorMsg;
