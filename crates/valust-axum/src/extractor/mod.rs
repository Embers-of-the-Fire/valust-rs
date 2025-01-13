//! [`axum`]-compatible extractor.

#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "json")]
pub use json::ValidJson;

#[cfg(feature = "form")]
pub mod form;
#[cfg(feature = "form")]
pub use form::ValidForm;

pub mod rejection;
