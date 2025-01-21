//! Type conversion functions for use in the `trans` attribute.

use std::str::FromStr;

/// Parses a string slice into the specified type `F`.
///
/// ```rust
/// # use valust_utils::convert::parse_to;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(func(String => try(parse_to::<i32>)))]
///     num: i32,
/// }
/// ```
pub fn parse_to<F: FromStr>(s: impl AsRef<str>) -> Result<F, F::Err> {
    s.as_ref().parse::<F>()
}

/// Converts a value of type `F` into type `T`.
///
/// ```rust
/// # use valust_utils::convert::into;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(func(&'static str => into))]
///     num: String,
/// }
/// ```
pub fn into<F: Into<T>, T>(f: F) -> T {
    f.into()
}

/// Tries to convert a value of type `F` into type `T`.
///
/// ```rust
/// # use valust_utils::convert::try_into;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(func(i32 => try(try_into)))]
///     num: i8,
/// }
/// ```
pub fn try_into<F: TryInto<T>, T>(f: F) -> Result<T, F::Error> {
    f.try_into()
}
