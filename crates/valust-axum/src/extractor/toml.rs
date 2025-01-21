//! Parse raw data from Toml with [`toml`] and validate it.

use std::ops::{Deref, DerefMut};

use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use serde::de::{DeserializeOwned, Error};
use valust::Validate;

use super::check_content_type;
use super::rejection::ValidateRejection;

/// Wrapper around a validated Toml value.
///
/// ```rust,no_run
/// use axum::{
///     routing::post,
///     Router,
/// };
/// use valust_axum::extractor::ValidToml;
/// use valust_derive::Valust;
/// use serde::Deserialize;
///
/// #[derive(Valust)]
/// // we need to derive `Deserialize` to use `ValidToml`
/// #[forward_derive(Deserialize)]
/// struct CreateUser {
///     #[valid(expr(email.contains("@")))]
///     email: String,
///     #[valid(expr(password.len() > 8))]
///     password: String,
/// }
///
/// async fn create_user(ValidToml(payload): ValidToml<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct ValidToml<T>(pub T);

impl<T> Deref for ValidToml<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ValidToml<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for ValidToml<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for ValidToml<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for ValidToml<T>
where
    S: Send + Sync,
    T: Validate,
    T::Raw: DeserializeOwned,
{
    type Rejection = ValidateRejection<toml::de::Error>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !check_content_type(req.headers(), "application/toml") {
            return Err(ValidateRejection::UnsupportedMediaType("application/toml"));
        }

        let raw = Bytes::from_request(req, state).await?;
        let raw = std::str::from_utf8(&raw).map_err(|e| {
            ValidateRejection::InvalidContentFormat(toml::de::Error::custom(e))
        })?;
        let data =
            toml::from_str(raw).map_err(ValidateRejection::InvalidContentFormat)?;

        let data = T::validate(data)?;

        Ok(ValidToml(data))
    }
}
