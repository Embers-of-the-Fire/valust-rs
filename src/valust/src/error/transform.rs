//! Transform errors.

use std::fmt::{self, Write};

use sealed::sealed;

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

#[sealed]
impl crate::error::display::ErrorDisplay for TransformError {
    fn full_display(&self, w: &mut impl Write) -> fmt::Result {
        if let Some(msg) = self.message {
            writeln!(w, "Transform error: {}", msg)?;
        } else {
            writeln!(w, "Transform error:")?;
        }
        writeln!(w, "Cause: {}", self.cause)?;
        writeln!(
            w,
            "Value: {}: {} = {}",
            self.field, self.source_type_name, self.value
        )?;
        writeln!(
            w,
            "Transformer: ({} => {}) {}",
            self.source_type_name, self.target_type_name, self.expression
        )?;
        writeln!(w, "Path: {}", self.path)?;
        writeln!(w)?;

        Ok(())
    }
}
