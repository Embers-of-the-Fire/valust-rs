//! Parse raw data from JSON with [`sonic`][sonic_rs] and validate it.

use std::ops::{Deref, DerefMut};

use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use valust::Validate;

use super::check_content_type;
use super::rejection::ValidateRejection;

/// Wrapper around a validated JSON value.
///
/// ```rust,no_run
/// use axum::{
///     routing::post,
///     Router,
/// };
/// use valust_axum::extractor::ValidSonic;
/// use valust_derive::Valust;
/// use serde::Deserialize;
///
/// #[derive(Valust)]
/// // we need to derive `Deserialize` to use `ValidSonic`
/// #[forward_derive(Deserialize)]
/// struct CreateUser {
///     #[valid(expr(email.contains("@")))]
///     email: String,
///     #[valid(expr(password.len() > 8))]
///     password: String,
/// }
///
/// async fn create_user(ValidSonic(payload): ValidSonic<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct ValidSonic<T>(pub T);

impl<T> Deref for ValidSonic<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ValidSonic<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for ValidSonic<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for ValidSonic<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for ValidSonic<T>
where
    S: Send + Sync,
    T: Validate,
    T::Raw: DeserializeOwned,
{
    type Rejection = ValidateRejection<sonic_rs::Error>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !check_content_type(req.headers(), "application/json") {
            return Err(ValidateRejection::UnsupportedMediaType("application/json"));
        }

        let raw = Bytes::from_request(req, state).await?;
        let raw = sonic_rs::from_slice(&raw)
            .map_err(ValidateRejection::InvalidContentFormat)?;

        let data = T::validate(raw)?;

        Ok(ValidSonic(data))
    }
}
