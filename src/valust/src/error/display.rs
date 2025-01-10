//! Display trait for error types.

use std::fmt::{self, Write};

use sealed::sealed;

/// In-one error display trait.
///
/// This trait is `pub` so that you can stringify the error,
/// but you're not supposed to implement this trait for external types.
#[sealed(pub(crate))]
pub trait ErrorDisplay {
    /// Full description of the error.
    fn full_display(&self, f: &mut impl Write) -> fmt::Result;

    /// Brief description of the error.
    fn brief_display(&self, f: &mut impl Write) -> fmt::Result {
        self.full_display(f)
    }

    /// Display the error fully and write it to a `String`.
    fn full_stringify(&self) -> String {
        let mut s = String::new();
        self.full_display(&mut s).unwrap();
        s
    }

    /// Display the error briefly and write it to a `String`.
    fn brief_stringify(&self) -> String {
        let mut s = String::new();
        self.brief_display(&mut s).unwrap();
        s
    }
}
