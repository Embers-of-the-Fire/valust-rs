//! Parse raw data from MessagePack with [`rmp-serde`][rmp_serde] and validate it.

use std::ops::{Deref, DerefMut};

use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use valust::Validate;

use super::check_content_type;
use super::rejection::ValidateRejection;

/// Wrapper around a validated [Message Pack](https://msgpack.org/) value.
///
/// ```rust,no_run
/// use axum::{
///     routing::post,
///     Router,
/// };
/// use valust_axum::extractor::ValidMsgPack;
/// use valust_derive::Valust;
/// use serde::Deserialize;
///
/// #[derive(Valust)]
/// // we need to derive `Deserialize` to use `ValidMsgPack`
/// #[forward_derive(Deserialize)]
/// struct CreateUser {
///     #[valid(email.contains("@"))]
///     email: String,
///     #[valid(password.len() > 8)]
///     password: String,
/// }
///
/// async fn create_user(ValidMsgPack(payload): ValidMsgPack<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct ValidMsgPack<T>(pub T);

impl<T> Deref for ValidMsgPack<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ValidMsgPack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for ValidMsgPack<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for ValidMsgPack<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for ValidMsgPack<T>
where
    S: Send + Sync,
    T: Validate,
    T::Raw: DeserializeOwned,
{
    type Rejection = ValidateRejection<rmp_serde::decode::Error>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !check_content_type(req.headers(), "application/msgpack") {
            return Err(ValidateRejection::UnsupportedMediaType(
                "application/msgpack",
            ));
        }

        let raw = Bytes::from_request(req, state).await?;
        let data = rmp_serde::from_slice(&raw)
            .map_err(ValidateRejection::InvalidContentFormat)?;

        let data = T::validate(data)?;

        Ok(ValidMsgPack(data))
    }
}
