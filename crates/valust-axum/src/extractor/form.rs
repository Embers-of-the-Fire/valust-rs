//! Parse raw data from `x-www-form-urlencoded` and validate it.

use std::ops::{Deref, DerefMut};

use axum::Form;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use valust::Validate;

use super::rejection::ValidateRejection;

/// Wrapper around a validated form value.
///
/// ## Response on Error
///
/// If the validation fails, the request will be rejected with a [`ValidateRejection`].
///
/// Specifically, if:
/// - The request body is not a valid `x-www-form-urlencoded` data, the rejection will be `Internal`.  
///   For detailed information, see [`Form`] for more information.
/// - The request body is a valid `x-www-form-urlencoded` data but the validation fails,
///   the rejection will be `Invalid`.  
///   For detailed information, see [`ValidationError`][valust::error::ValidationError] for more information.
///
/// ## Example
///
/// ```rust,no_run
/// use axum::{
///     routing::post,
///     Router,
/// };
/// use valust_axum::extractor::ValidForm;
/// use valust_derive::Valust;
/// use serde::Deserialize;
///
/// #[derive(Valust)]
/// // we need to derive `Deserialize` to use `ValidForm`
/// #[forward_derive(Deserialize)]
/// struct CreateUser {
///     #[valid(expr(email.contains("@")))]
///     email: String,
///     #[valid(expr(password.len() > 8))]
///     password: String,
/// }
///
/// async fn create_user(ValidForm(payload): ValidForm<CreateUser>) {
///     // payload is a `CreateUser`
/// }
///
/// let app = Router::new().route("/users", post(create_user));
/// # let _: Router = app;
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct ValidForm<T>(pub T);

impl<T> Deref for ValidForm<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ValidForm<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for ValidForm<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T, S> FromRequest<S> for ValidForm<T>
where
    S: Send + Sync,
    T: Validate,
    T::Raw: DeserializeOwned,
{
    type Rejection = ValidateRejection<<Form<T::Raw> as FromRequest<S>>::Rejection>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(raw) = Form::<T::Raw>::from_request(req, state)
            .await
            .map_err(ValidateRejection::InvalidContentFormat)?;

        let data = T::validate(raw)?;

        Ok(ValidForm(data))
    }
}
