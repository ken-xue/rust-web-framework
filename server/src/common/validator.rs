use async_trait::async_trait;
use axum::{extract::{rejection::FormRejection, FromRequest}, http::StatusCode, response::{IntoResponse, Response}, BoxError, Json};
use axum::body::HttpBody;
use axum::extract::rejection::JsonRejection;
use axum::http::{Request};
use serde::{de::DeserializeOwned};
use thiserror::Error;
use validator::Validate;
use crate::common::response::error;

#[derive(Debug, Clone, Copy, Default)]
pub struct Validated<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Validated<T>
    where
        T: DeserializeOwned + Validate,
        B: HttpBody + Send + 'static,
        B::Data: Send,
        B::Error: Into<BoxError>,
        S: Send + Sync,
{
    type Rejection = ServerError;
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, _state).await?;
        value.validate()?;
        Ok(Validated(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                // let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                error(Box::try_from(e).unwrap())
                // (StatusCode::BAD_REQUEST, message)
            }
            // ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ServerError::AxumJsonRejection(e) => error(Box::try_from(e).unwrap()),
        }.into_response()
    }
}