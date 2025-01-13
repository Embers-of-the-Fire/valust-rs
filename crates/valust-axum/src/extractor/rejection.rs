//! Rejection types for wrapping the `valust` error type.

use axum::extract::FromRequest;
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
pub enum ValidateRejection<T: FromRequest<S>, S: Sync + Send> {
    /// The request data was invalid.
    Invalid(ValidationError),
    /// An internal rejection occurred.
    Internal(T::Rejection),
}

impl<T: FromRequest<S>, S: Sync + Send> IntoResponse for ValidateRejection<T, S> {
    fn into_response(self) -> Response {
        match self {
            Self::Invalid(err) => {
                (StatusCode::BAD_REQUEST, err.human_readable_stringify())
                    .into_response()
            }
            Self::Internal(rej) => rej.into_response(),
        }
    }
}
