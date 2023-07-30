use async_trait::async_trait;
use axum::{extract::{rejection::FormRejection, FromRequest}, http::StatusCode, response::{IntoResponse, Response}, BoxError, Json};
use axum::body::HttpBody;
use axum::extract::rejection::JsonRejection;
use axum::http::{Request};
use serde::{de::DeserializeOwned};
use thiserror::Error;
use validator::Validate;
use crate::common::response::error;
use crate::system::auth::Resp;

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
        let (status, error_message) = match self {
            ServerError::ValidationError(e) => (1, format!("Input validation error: [{}]", e).replace('\n', ", ")),
            ServerError::AxumJsonRejection(e) => (2, e.to_string()),
        };
        let bd: Resp = Resp {
            code: status,
            message: error_message.to_string(),
        };
        (StatusCode::OK, Json(bd)).into_response()
    }
}