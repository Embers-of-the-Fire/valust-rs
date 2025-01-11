//! Type conversion functions for use in the `trans` attribute.

use std::str::FromStr;

/// Parses a string slice into the specified type `F`.
///
/// ```rust,ignore
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(try(String => fn(parses_to::<i32>)))]
///     num: i32,
/// }
/// ```
pub fn parse_to<F: FromStr>(s: impl AsRef<str>) -> Result<F, F::Err> {
    s.as_ref().parse::<F>()
}

/// Converts a value of type `F` into type `T`.
///
/// ```rust,ignore
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(A => fn(into::<B>))]
///     num: B,
/// }
/// ```
pub fn into<F: Into<T>, T>(f: F) -> T {
    f.into()
}

/// Tries to convert a value of type `F` into type `T`.
///
/// ```rust,ignore
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(try(A => fn(try_into::<B>)))]
///     num: Result<B, String>,
/// }
/// ```
pub fn try_into<F: TryInto<T>, T>(f: F) -> Result<T, F::Error> {
    f.try_into()
}
