//! Parse raw data from JSON and validate it.

use std::ops::{Deref, DerefMut};

use axum::Json;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use valust::Validate;

use super::rejection::ValidateRejection;

/// Wrapper around a validated JSON value.
///
/// ```rust,no_run
/// use axum::{
///     routing::post,
///     Router,
/// };
/// use valust_axum::extractor::ValidJson;
/// use valust_derive::Valust;
/// use serde::Deserialize;
///
/// #[derive(Valust)]
/// // we need to derive `Deserialize` to use `ValidJson`
/// #[forward_derive(Deserialize)]
/// struct CreateUser {
///     #[valid(email.contains("@"))]
///     email: String,
///     #[valid(password.len() > 8)]
///     password: String,
/// }
///
/// async fn create_user(ValidJson(payload): ValidJson<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct ValidJson<T>(pub T);

impl<T> Deref for ValidJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ValidJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for ValidJson<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for ValidJson<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    T: Validate,
    T::Raw: DeserializeOwned,
{
    type Rejection = ValidateRejection<Json<T::Raw>, S>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(raw) = Json::<T::Raw>::from_request(req, state)
            .await
            .map_err(ValidateRejection::Internal)?;

        let data = T::validate(raw).map_err(ValidateRejection::Invalid)?;

        Ok(ValidJson(data))
    }
}
