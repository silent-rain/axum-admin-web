//! Json extractor.
use std::ops::Deref;

use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::{header, HeaderMap},
};

use serde::de::DeserializeOwned;
use validator::Validate;

use axum_response::ResponseErr;
use code::Error;

#[derive(Debug)]
pub struct Json<T>(pub T);

#[allow(unused)]
impl<T> Json<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<S, T> FromRequest<S> for Json<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = ResponseErr;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !json_content_type(req.headers()) {
            return Err(ResponseErr::new(Error::HeaderContentType(
                "expected request with `Content-Type: application/json`".to_string(),
            )));
        }

        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|e| Error::InvalidParameter(e.to_string()))?;

        // 获取body数据
        let body: T = serde_json::from_slice(&bytes)
            .map_err(|e| Error::JsonDeserialization(e.to_string()))?;

        // 验证 body 数据
        body.validate()
            .map_err(|e| Error::ValidateError(e.to_string()))?;
        Ok(Json(body))
    }
}

fn json_content_type(headers: &HeaderMap) -> bool {
    let content_type = if let Some(content_type) = headers.get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|name| name == "json"));

    is_json_content_type
}
