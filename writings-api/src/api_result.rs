use axum::http::StatusCode;
use strum::Display;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(thiserror::Error, Debug, Display)]
pub enum ApiError {
    NotFound,
    Axum(#[from] axum::Error),
    Io(#[from] std::io::Error),
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND.into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
        }
    }
}
