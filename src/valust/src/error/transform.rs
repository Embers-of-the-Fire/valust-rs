//! Transform errors.

use super::ErrorShow;

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
