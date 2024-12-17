//! 接口响应类型
use code::Error;

use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

/// 响应体封装
pub type Responder<T> = Result<Response<T>, Response<T>>;

/// Data list
#[derive(Debug, Serialize, Deserialize)]
struct DataList<T: Serialize> {
    data_list: Vec<T>,
    total: u64,
}

/// Data list wrapper
#[derive(Debug, Serialize, Deserialize)]
enum DataWrapper<T: Serialize> {
    Data(T),
    List(DataList<T>),
}

/// Response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    /// Return business code
    code: u16,
    /// Return message
    msg: String,
    /// Return data
    data: Option<DataWrapper<T>>,
}

impl<T: Serialize> Default for Response<T> {
    fn default() -> Self {
        Self {
            code: Error::OK.code(),
            msg: Error::OK.msg(),
            data: None,
        }
    }
}

impl<T: Serialize> Response<T> {
    /// Return success with no data
    pub fn ok() -> Self {
        Self {
            data: None,
            ..Default::default()
        }
    }

    /// Set return data
    pub fn data(data: T) -> Self {
        Self {
            data: Some(DataWrapper::Data(data)),
            ..Default::default()
        }
    }

    /// Set return data list
    pub fn data_list(data_list: Vec<T>, total: u64) -> Self {
        Self {
            data: Some(DataWrapper::List(DataList { data_list, total })),
            ..Default::default()
        }
    }

    ///  Set return msg
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    /// Add msg information and add new information based on the error code information
    pub fn append_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg);
        self
    }
}

/// 打印 Response
impl<T: Serialize> std::fmt::Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(response code: {}, msg: {})", self.code, self.msg)
    }
}

/// 将错误枚举转换为响应体
impl<T: Serialize> From<Error> for Response<T> {
    fn from(err: Error) -> Self {
        Response {
            code: err.code(),
            msg: err.msg(),
            ..Default::default()
        }
    }
}

/// Axum 响应体实现
impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Book {
        title: String,
        desc: String,
    }

    #[test]
    fn test_ok() {
        let resp = Response::<()>::ok().with_msg("ok");
        println!("resp: {:#?}", resp);

        let resp_str = serde_json::to_string(&resp).expect("Failed to serialize response to JSON");
        println!("resp str: {:#?}", resp_str);
    }

    #[test]
    fn test_data() {
        let book = Book {
            title: "weather".to_string(),
            desc: "This is a book about weather".to_string(),
        };

        let resp = Response::data(book).with_msg("operation successful");
        println!("resp: {:#?}", resp);

        let resp_str = serde_json::to_string(&resp).expect("Failed to serialize response to JSON");
        println!("resp str: {:#?}", resp_str);
    }

    #[test]
    fn test_data_list() {
        let books = vec![Book {
            title: "Douqi Continent".to_string(),
            desc: "This is a fantasy novel".to_string(),
        }];
        let total = books.len() as u64;

        let resp = Response::data_list(books, total).with_msg("operation successful");
        println!("resp: {:#?}", resp);

        let resp_str = serde_json::to_string(&resp).expect("Failed to serialize response to JSON");
        println!("resp str: {:#?}", resp_str);
    }

    #[test]
    fn test_into_response() {
        let book = Book {
            title: "weather".to_string(),
            desc: "This is a book about weather".to_string(),
        };

        let resp = Response::data(book)
            .with_msg("operation successful")
            .into_response();
        println!("resp: {:#?}", resp);
    }
}
