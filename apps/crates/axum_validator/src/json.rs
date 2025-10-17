//! Json extractor.
use std::ops::Deref;

use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::{HeaderMap, header},
};

use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::Error;

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
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !json_content_type(req.headers()) {
            return Err(Error::HeaderContentType(
                "expected request with `Content-Type: application/json`".to_string(),
            ));
        }

        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|e| Error::InvalidParameter(e.to_string()))?;
        // 获取body数据
        let body: T =
            serde_json::from_slice(&bytes).map_err(|e| Error::SerdeJsonError(e.to_string()))?;

        // 验证 body 数据
        body.validate()?;
        Ok(Json(body))
    }
}

fn json_content_type(headers: &HeaderMap) -> bool {
    let content_type = match headers.get(header::CONTENT_TYPE) {
        Some(content_type) => content_type,
        _ => {
            return false;
        }
    };

    let content_type = match content_type.to_str() {
        Ok(content_type) => content_type,
        Err(_) => return false,
    };

    let mime = match content_type.parse::<mime::Mime>() {
        Ok(mime) => mime,
        Err(_) => return false,
    };

    mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().is_some_and(|name| name == "json"))
}
