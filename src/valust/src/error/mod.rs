//! Human-readable error produced by `valust`.

pub mod display;
pub mod transform;
pub mod validate;

use std::fmt::{self, Debug, Display, Write};

use sealed::sealed;
use transform::TransformError;
use validate::ValidateError;

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

/// Type alias for `Result<ValidationError>`.
pub type ValidationResult<T> = Result<T, ValidationError>;

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

#[sealed]
impl display::ErrorDisplay for ValidationError {
    fn full_display(&self, w: &mut impl Write) -> fmt::Result {
        self.validates.iter().try_for_each(|t| t.full_display(w))?;
        self.transforms.iter().try_for_each(|t| t.full_display(w))?;

        Ok(())
    }

    fn brief_display(&self, w: &mut impl Write) -> fmt::Result {
        self.validates.iter().try_for_each(|t| t.brief_display(w))?;
        self.transforms
            .iter()
            .try_for_each(|t| t.brief_display(w))?;

        Ok(())
    }

    fn human_readable_display(&self, w: &mut impl Write) -> fmt::Result {
        write!(w, "❌ Oops! Some of the values are invalid!\n\n")?;
        let mut cnt = 0;
        self.validates.iter().try_for_each(|t| {
            cnt += 1;
            write!(w, "{: <4}", cnt.to_string() + ".")?;
            t.human_readable_display(w)?;
            writeln!(w)?;
            Ok(())
        })?;
        self.transforms.iter().try_for_each(|t| {
            cnt += 1;
            write!(w, "{: <4}", cnt.to_string() + ".")?;
            t.human_readable_display(w)?;
            writeln!(w)?;
            Ok(())
        })?;

        Ok(())
    }
}
