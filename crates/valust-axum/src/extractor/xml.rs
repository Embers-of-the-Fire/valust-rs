//! Parse raw data from XML with [`quick-xml`][quick_xml] and validate it.

use std::ops::{Deref, DerefMut};

use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use serde::de::{DeserializeOwned, Error};
use valust::Validate;

use super::check_content_type;
use super::rejection::ValidateRejection;

/// Wrapper around a validated Xml value.
///
/// ```rust,no_run
/// use axum::{
///     routing::post,
///     Router,
/// };
/// use valust_axum::extractor::ValidXml;
/// use valust_derive::Valust;
/// use serde::Deserialize;
///
/// #[derive(Valust)]
/// // we need to derive `Deserialize` to use `ValidXml`
/// #[forward_derive(Deserialize)]
/// struct CreateUser {
///     #[valid(email.contains("@"))]
///     email: String,
///     #[valid(password.len() > 8)]
///     password: String,
/// }
///
/// async fn create_user(ValidXml(payload): ValidXml<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct ValidXml<T>(pub T);

impl<T> Deref for ValidXml<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ValidXml<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for ValidXml<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for ValidXml<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for ValidXml<T>
where
    S: Send + Sync,
    T: Validate,
    T::Raw: DeserializeOwned,
{
    type Rejection = ValidateRejection<quick_xml::DeError>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !check_content_type(req.headers(), "application/xml") {
            return Err(ValidateRejection::UnsupportedMediaType("application/xml"));
        }

        let raw = Bytes::from_request(req, state).await?;
        let raw = std::str::from_utf8(&raw).map_err(|e| {
            ValidateRejection::InvalidContentFormat(quick_xml::DeError::custom(e))
        })?;
        let data = quick_xml::de::from_str(raw)
            .map_err(ValidateRejection::InvalidContentFormat)?;

        let data = T::validate(data)?;

        Ok(ValidXml(data))
    }
}
