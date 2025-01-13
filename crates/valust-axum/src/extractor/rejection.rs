//! Rejection types for wrapping the `valust` error type.

use axum::extract::rejection::BytesRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use valust::error::ValidationError;
use valust::error::display::ErrorDisplay;

/// Rejection type for a validated extractor.
///
/// ## Response on Error
///
/// If the validation fails, the request will be rejected with a `ValidateRejection`.
///
/// Specifically, if:
/// - The request data is not valid, the rejection will be `Invalid`.
///   And the response will be a `400 Bad Request` with the error message
///   in [plain text][plain-display].
/// - An internal rejection occurs, the rejection will be `Internal`.
///   And the response will be the rejection response, depending on the rejection's
///   implementation.
///
/// [plain-display]: valust::error::display::ErrorDisplay#method.human_readable_display
#[derive(Debug)]
pub enum ValidateRejection<E> {
    /// The `Invalid` variant represents an HTTP 422 Unprocessable Entity error.
    /// This error occurs when the data provided is not valid.
    InvalidValue(ValidationError),
    /// The `UnsupportedMediaType` variant represents an HTTP 415 Unsupported Media Type error.
    /// This error occurs when the server cannot handle the media type specified in the request.
    UnsupportedMediaType(&'static str),
    /// Failed to buffer body.
    BytesRejection(BytesRejection),
    /// The `InvalidContentFormat` variant represents an HTTP 422 Unprocessable Content error with an associated error message.
    /// This error occurs when the server cannot handle the content in the request.
    InvalidContentFormat(E),
}

impl<R: ToString> IntoResponse for ValidateRejection<R> {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidValue(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                err.human_readable_stringify(),
            )
                .into_response(),
            Self::UnsupportedMediaType(media) => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                format!("Unsupported media type: {}", media),
            )
                .into_response(),
            Self::BytesRejection(err) => err.into_response(),
            Self::InvalidContentFormat(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Invalid content format: {}", err.to_string()),
            )
                .into_response(),
        }
    }
}

impl<E> From<ValidationError> for ValidateRejection<E> {
    fn from(err: ValidationError) -> Self {
        Self::InvalidValue(err)
    }
}

impl<E> From<BytesRejection> for ValidateRejection<E> {
    fn from(err: BytesRejection) -> Self {
        Self::BytesRejection(err)
    }
}
