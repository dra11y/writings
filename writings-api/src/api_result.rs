use axum::http::StatusCode;
use strum::Display;

pub type WritingsApiResult<T> = Result<T, WritingsApiError>;

#[derive(thiserror::Error, Debug, Display)]
pub enum WritingsApiError {
    NotFound,
    Axum(#[from] axum::Error),
    Io(#[from] std::io::Error),
}

impl axum::response::IntoResponse for WritingsApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            WritingsApiError::NotFound => StatusCode::NOT_FOUND.into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
        }
    }
}
