use thiserror::Error;

/// Alias for [`Result`] with [`WritingsError`] as the Error type.
pub type WritingsResult<T> = Result<T, WritingsError>;

/// An error type specific for this crate.
#[derive(Debug, Error)]
pub enum WritingsError {
    #[error("serde deserialize {0}")]
    SerdeValue(#[from] serde::de::value::Error),
}
