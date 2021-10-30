use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Core error types.
#[derive(Error, Debug)]
pub enum Error {
    /// A request validation failed
    #[error("Request validation failed with {0}")]
    RequestValidation(String),

    /// Common error case when requesting parsing into own structs
    #[error("Error deserializing response")]
    SerdeError(#[source] serde_json::Error),

    /// Http based error
    #[error("HttpError: {0}")]
    HttpError(#[source] http::Error),
}

/// An error response from the API.
#[derive(Error, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[error("{message}: {reason}")]
pub struct ErrorResponse {
    /// The status
    pub status: String,
    /// A message about the error
    #[serde(default)]
    pub message: String,
    /// The reason for the error
    #[serde(default)]
    pub reason: String,
    /// The error code
    pub code: u16,
}
