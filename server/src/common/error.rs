// use axum::http::StatusCode;
// use axum::Json;
// use axum::response::{IntoResponse, Response};
// use serde_json::json;
// use crate::system::user::repo::UserRepoError;
//
// /// Our app's top level error type.
// pub enum AppError {
//     /// Something went wrong when calling the user repo.
//     UserRepo(UserRepoError),
//     NotFound(u64,String)
// }
//
// /// This makes it possible to use `?` to automatically convert a `UserRepoError`
// /// into an `AppError`.
// impl From<UserRepoError> for AppError {
//     fn from(inner: UserRepoError) -> Self {
//         AppError::UserRepo(inner)
//     }
// }
//
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             AppError::UserRepo(UserRepoError::NotFound) => {
//                 (StatusCode::NOT_FOUND, "User not found")
//             }
//             AppError::UserRepo(UserRepoError::InvalidUsername) => {
//                 (StatusCode::UNPROCESSABLE_ENTITY, "Invalid username")
//             }
//         };
//
//         let body = Json(json!({"error": error_message}));
//
//         (status, body).into_response()
//     }
// }


use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::common::response;
use crate::common::response::response;

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let err_response = response::ErrorResponse{
            code: 5000,
            message: format!("{}", self.0)
        };
        (
            StatusCode::OK,
            Json(err_response),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
