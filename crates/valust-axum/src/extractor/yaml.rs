//! Parse raw data from YAML with [`serde_yaml`] and validate it.

use std::ops::{Deref, DerefMut};

use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use valust::Validate;

use super::check_content_type;
use super::rejection::ValidateRejection;

/// Wrapper around a validated YAML value.
///
/// ```rust,no_run
/// use axum::{
///     routing::post,
///     Router,
/// };
/// use valust_axum::extractor::ValidYaml;
/// use valust_derive::Valust;
/// use serde::Deserialize;
///
/// #[derive(Valust)]
/// // we need to derive `Deserialize` to use `ValidYaml`
/// #[forward_derive(Deserialize)]
/// struct CreateUser {
///     #[valid(expr(email.contains("@")))]
///     email: String,
///     #[valid(expr(password.len() > 8))]
///     password: String,
/// }
///
/// async fn create_user(ValidYaml(payload): ValidYaml<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct ValidYaml<T>(pub T);

impl<T> Deref for ValidYaml<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ValidYaml<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for ValidYaml<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for ValidYaml<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for ValidYaml<T>
where
    S: Send + Sync,
    T: Validate,
    T::Raw: DeserializeOwned,
{
    type Rejection = ValidateRejection<serde_yaml::Error>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !check_content_type(req.headers(), "application/yaml") {
            return Err(ValidateRejection::UnsupportedMediaType("application/yaml"));
        }

        let raw = Bytes::from_request(req, state).await?;
        let data = serde_yaml::from_slice(&raw)
            .map_err(ValidateRejection::InvalidContentFormat)?;

        let data = T::validate(data)?;

        Ok(ValidYaml(data))
    }
}
