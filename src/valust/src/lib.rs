//! # Valust
//!
//! The `Valust` crate provides a general data validation interface (trait
//! `Validate` and the derived trait `ValidateFrom`).

pub mod error;

#[cfg(feature = "derive")]
pub use valust_derive as derive;

/// The `Validate` trait provides a general data validation interface.
///
/// The `Validate` trait is used to validate raw data and convert it into a
/// validated data type. The raw data is passed to the `validate` function,
/// which returns a `Result` containing either the validated data or an error.
pub trait Validate: Sized {
    /// The raw data type.
    type Raw;

    /// Validates the raw data and returns the validated data or an error.
    fn validate(raw: Self::Raw) -> Result<Self, error::ValidationError>;
}

/// A type alias for the raw data type of a validated data type.
///
/// This type alias is used to simplify the definition of the `Raw` associated
/// type in the `Validate` trait.
pub type Raw<T> = <T as Validate>::Raw;
