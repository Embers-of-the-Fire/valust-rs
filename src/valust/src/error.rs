//! Human-readable error produced by `valust`.

use std::fmt::{Debug, Display, Formatter};

/// Display-able error trait.
///
/// You don't need to manually implement this trait as this trait
/// has been implemented for all `Debug + Display` types.
pub trait ErrorShow: Debug + Display {}

impl<T: Debug + Display> ErrorShow for T {}

/// Any validation error.
#[derive(Debug, Default)]
pub struct ValidationError {
    /// Error produced by validators.
    pub validates: Vec<ValidateError>,
    /// Error produced by transformers.
    pub transforms: Vec<TransformError>,
}

impl ValidationError {
    /// Create an empty error set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if the error set contains any error instance.
    pub fn check(self) -> Result<(), ValidationError> {
        if self.validates.is_empty() && self.transforms.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }

    /// Push a validator error to the set.
    pub fn push_validate_error(&mut self, err: ValidateError) {
        self.validates.push(err);
    }

    /// Push a transformer error to the set.
    pub fn push_transform_error(&mut self, err: TransformError) {
        self.transforms.push(err);
    }

    /// Extend the set.
    ///
    /// This will modify original set's `path` field.
    pub fn extend_error(&mut self, parent: &str, rhs: Self) {
        self.validates
            .extend(rhs.validates.into_iter().map(|mut x| {
                x.path = format!("{}.{}", parent, x.path);
                x
            }));
        self.transforms
            .extend(rhs.transforms.into_iter().map(|mut x| {
                x.path = format!("{}.{}", parent, x.path);
                x
            }));
    }
}

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

#[derive(Debug)]
/// Represents an error that occurs during a transformation process.
pub struct TransformError {
    /// The name of the field where the error occurred.
    pub field: &'static str,
    /// The path to the field where the error occurred.
    pub path: String,
    /// The value that caused the error.
    pub value: String,
    /// The underlying cause of the error.
    pub cause: Box<dyn ErrorShow + 'static>,
    /// An optional message providing additional information about the error.
    pub message: Option<&'static str>,
    /// The expression that caused the error.
    pub expression: &'static str,
    /// The name of the source type involved in the transformation.
    pub source_type_name: &'static str,
    /// The name of the target type involved in the transformation.
    pub target_type_name: &'static str,
}

/// A custom error type that holds a static string slice.
///
/// This struct is used to represent errors with a static message.
#[derive(Debug, Clone)]
pub struct CustomError(
    /// A static string slice that describes the error.
    pub &'static str,
);

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
