//! Validate errors.

use std::fmt::{self, Write};

use sealed::sealed;

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
    /// If there's no error found (e.g. the value is simply invalid), then the field will be `None`.
    pub cause: Option<Box<dyn ErrorShow + 'static>>,
    /// An optional message providing additional information about the error.
    pub message: Option<&'static str>,
    /// The expression that was evaluated and caused the error.
    pub expression: &'static str,
    /// he type name of the value that caused the error.
    pub type_name: &'static str,
}

#[sealed]
impl crate::error::display::ErrorDisplay for ValidateError {
    fn full_display(&self, w: &mut impl Write) -> fmt::Result {
        if let Some(msg) = self.message {
            writeln!(w, "Validate error: {}", msg)?;
        } else {
            writeln!(w, "Validate error:")?;
        }
        if let Some(cause) = &self.cause {
            writeln!(w, "Cause: {}", cause)?;
        }
        writeln!(
            w,
            "Value: {}: {} = {}",
            self.field, self.type_name, self.value
        )?;
        writeln!(w, "Validator: {}", self.expression)?;
        writeln!(w, "Path: {}", self.path)?;
        writeln!(w)?;

        Ok(())
    }

    fn brief_display(&self, w: &mut impl Write) -> fmt::Result {
        write!(w, "Validate error [{}]", self.path)?;
        if let Some(msg) = self.message {
            writeln!(w, ": {}", msg)?;
        } else if let Some(cause) = &self.cause {
            writeln!(w, ": {}", cause)?;
        }

        Ok(())
    }

    fn human_readable_display(&self, w: &mut impl Write) -> fmt::Result {
        write!(w, "Validate: ")?;
        if let Some(msg) = self.message {
            writeln!(w, "{}", msg)?;
        } else if let Some(cause) = &self.cause {
            writeln!(w, "{}", cause)?;
        } else {
            writeln!(w, "Invalid value found.")?;
        }
        writeln!(w, "Backtrace:")?;
        writeln!(w, "    Value: {}", self.value)?;
        writeln!(w, "    Operation: {}", self.expression)?;
        if let Some(cause) = &self.cause {
            writeln!(w, "    Error: {:?}", cause)?;
        }

        Ok(())
    }
}
