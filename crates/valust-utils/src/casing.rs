//! Case-converting utilities.

pub use convert_case as case;
pub use convert_case::Case;
use convert_case::Casing;

/// Converts a string to a specified case.
///
/// ```rust,ignore
/// use valust_utils::casing::{Case, to_case};
/// use valust_derive::Valust;
///
/// #[derive(Valust)]
/// struct Casing(
///     #[trans(fn(to_case(Case::Camel)))]
///     String
/// );
/// ```
pub fn to_case<A: AsRef<str>>(case: Case) -> impl Fn(A) -> String {
    move |s| s.as_ref().to_case(case)
}

/// Converts a string to `Upper`.
///
/// Alias for `to_case(Case::Upper)`.
///
/// Uppercase strings are delimited by spaces and all characters are uppercase.
pub fn to_upper(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Upper)
}

/// Converts a string to `Lower`.
///
/// Alias for `to_case(Case::Lower)`.
///
/// Lowercase strings are delimited by spaces and all characters are lowercase.
pub fn to_lower(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Lower)
}

/// Converts a string to `Title`.
///
/// Alias for `to_case(Case::Title)`.
///
/// Title case strings are delimited by spaces. Only the leading character of each word is uppercase.
pub fn to_title(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Title)
}

/// Converts a string to `Sentence`.
///
/// Alias for `to_case(Case::Sentence)`.
///
/// Sentence case strings are delimited by spaces. Only the leading character of the first word is uppercase.
pub fn to_sentence(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Sentence)
}

/// Converts a string to `Toggle`.
///
/// Alias for `to_case(Case::Toggle)`.
///
/// Toggle case strings are delimited by spaces. All characters are uppercase except for the leading character of each word, which is lowercase.
pub fn to_toggle(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Toggle)
}

/// Converts a string to `Camel`.
///
/// Alias for `to_case(Case::Camel)`.
///
/// Camel case strings are lowercase, but for every word except the first the first letter is capitalized.
pub fn to_camel(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Camel)
}

/// Converts a string to `Pascal`.
///
/// Alias for `to_case(Case::Pascal)`.
///
/// Pascal case strings are lowercase, but for every word the first letter is capitalized.
pub fn to_pascal(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Pascal)
}

/// Converts a string to `Snake`.
///
/// Alias for `to_case(Case::Snake)`.
///
/// Snake case strings are delimited by underscores `_` and are all lowercase.
pub fn to_snake(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Snake)
}

/// Converts a string to `Constant`.
///
/// Alias for `to_case(Case::Constant)`.
///
/// Constant case strings are delimited by underscores `_` and are all uppercase.
pub fn to_constant(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Constant)
}

/// Converts a string to `Kebab`.
///
/// Alias for `to_case(Case::Kebab)`.
///
/// Kebab case strings are delimited by hyphens `-` and are all lowercase.
pub fn to_kebab(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Kebab)
}

/// Converts a string to `Cobol`.
///
/// Alias for `to_case(Case::Cobol)`.
///
/// Cobol case strings are delimited by hyphens `-` and are all uppercase.
pub fn to_cobol(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Cobol)
}

/// Converts a string to `Train`.
///
/// Alias for `to_case(Case::Train)`.
///
/// Train case strings are delimited by hyphens `-`. All characters are lowercase except for the leading character of each word.
pub fn to_train(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Train)
}

/// Converts a string to `Flat`.
///
/// Alias for `to_case(Case::Flat)`.
///
/// Flat case strings are all lowercase, with no delimiter. Note that word boundaries are lost.
pub fn to_flat(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Flat)
}

/// Converts a string to `UpperFlat`.
///
/// Alias for `to_case(Case::UpperFlat)`.
///
/// Upper flat case strings are all uppercase, with no delimiter. Note that word boundaries are lost.
pub fn to_upper_flat(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::UpperFlat)
}

/// Converts a string to `Alternating`.
///
/// Alias for `to_case(Case::Alternating)`.
///
/// Alternating case strings are delimited by spaces. Characters alternate between uppercase and lowercase.
pub fn to_alternating(s: impl AsRef<str>) -> String {
    s.as_ref().to_case(Case::Alternating)
}
