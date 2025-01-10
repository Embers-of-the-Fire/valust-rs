//! # Valust
//!
//! The `Valust` crate provides a general data validation interface (trait
//! `Validate` and the derived trait `ValidateFrom`).

pub mod error;

#[cfg(feature = "derive")]
pub use valust_derive as derive;

/// A trait for validating and converting raw input data into a specific type.
///
/// **Note:** This trait will automatically implements `ValidateFrom<Self>` for `Output`.
///
/// # Type Parameters
///
/// - `Output` - The type of the output validated data.
///
/// # Errors
///
/// This method returns a `Result` with the implementing type on success, or a `ValidationError` on failure.
pub trait Validate<Output> {
    /// Validate the raw data.
    fn validate(self) -> Result<Output, error::ValidationError>;
}

/// A trait for validating and converting raw input data into a specific type.
///
/// **Note:** This trait will be automatically implemented if `R` implements `Validate<Self>`.
///
/// # Type Parameters
///
/// - `R` - The type of the raw input data to be validated and converted.
///
/// # Errors
///
/// This method returns a `Result` with the implementing type on success, or a `ValidationError` on failure.
pub trait ValidateFrom<R> {
    /// Validate the raw data.
    fn validate_from(raw: R) -> Result<Self, error::ValidationError>
    where
        Self: Sized;
}

impl<T, U> ValidateFrom<T> for U
where
    T: Validate<U>,
{
    fn validate_from(raw: T) -> Result<Self, error::ValidationError> {
        raw.validate()
    }
}
