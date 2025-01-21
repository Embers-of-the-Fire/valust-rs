//! Parse raw data from CBOR with [`ciborium`] and validate it.

use std::ops::{Deref, DerefMut};

use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use valust::Validate;

use super::check_content_type;
use super::rejection::ValidateRejection;

/// Wrapper around a validated CBOR value.
///
/// ```rust,no_run
/// use axum::{
///     routing::post,
///     Router,
/// };
/// use valust_axum::extractor::ValidCbor;
/// use valust_derive::Valust;
/// use serde::Deserialize;
///
/// #[derive(Valust)]
/// // we need to derive `Deserialize` to use `ValidCbor`
/// #[forward_derive(Deserialize)]
/// struct CreateUser {
///     #[valid(expr(email.contains("@")))]
///     email: String,
///     #[valid(expr(password.len() > 8))]
///     password: String,
/// }
///
/// async fn create_user(ValidCbor(payload): ValidCbor<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct ValidCbor<T>(pub T);

impl<T> Deref for ValidCbor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ValidCbor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for ValidCbor<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for ValidCbor<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for ValidCbor<T>
where
    S: Send + Sync,
    T: Validate,
    T::Raw: DeserializeOwned,
{
    type Rejection = ValidateRejection<ciborium::de::Error<std::io::Error>>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !check_content_type(req.headers(), "application/cbor") {
            return Err(ValidateRejection::UnsupportedMediaType("application/cbor"));
        }

        let raw = Bytes::from_request(req, state).await?;
        let data = ciborium::from_reader(&raw as &[u8])
            .map_err(ValidateRejection::InvalidContentFormat)?;

        let data = T::validate(data)?;

        Ok(ValidCbor(data))
    }
}
