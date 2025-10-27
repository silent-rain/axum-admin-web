//! Path extractor.

use std::ops::Deref;

use axum::{extract::FromRequestParts, http::request::Parts};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::Error;

/// A wrapper around `Axum Path` that validates the extracted path parameters.
#[derive(Debug)]
pub struct Path<T>(pub T);

impl<T> Path<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Path<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Path<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, S> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path =
            <axum::extract::Path<T> as axum::extract::FromRequestParts<S>>::from_request_parts(
                parts, state,
            )
            .await
            .map_err(|e| Error::PathRejection(e.to_string()))?;
        path.0.validate()?;
        Ok(Path(path.0))
    }
}
