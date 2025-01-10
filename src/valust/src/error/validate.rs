//! Validate errors.

use std::fmt::{Display, Formatter};

use super::ErrorShow;

/// Represents an error that occurs during validation.
#[derive(Debug)]
pub struct ValidateError {
    /// The name of the field that caused the error.
    pub field: &'static str,
    /// The path to the field that caused the error.
    pub path: String,
    /// The value that caused the error.
    ///
    /// The value is formatted in the macro-generated code, and you can modify the
    /// behavior by configuring the derive-macro.
    pub value: String,
    /// The underlying cause of the error, implementing the `ErrorShow` trait.
    ///
    /// This will fall back to [`ValidateFail`] if the validator simply evaluates to `false`.
    pub cause: Box<dyn ErrorShow + 'static>,
    /// An optional message providing additional information about the error.
    pub message: Option<&'static str>,
    /// The expression that was evaluated and caused the error.
    pub expression: &'static str,
    /// he type name of the value that caused the error.
    pub type_name: &'static str,
}

/// A placeholder type for validators evaluating to `false`.
#[derive(Debug, Clone, Copy)]
pub struct ValidateFail;

impl Display for ValidateFail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "validate expression evaluate to `false`")
    }
}
