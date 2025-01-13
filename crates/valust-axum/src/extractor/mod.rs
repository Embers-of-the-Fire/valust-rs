//! [`axum`]-compatible extractor.

#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub mod json;
use axum::http::{header, HeaderMap};
#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub use json::ValidJson;

#[cfg(feature = "form")]
#[cfg_attr(docsrs, doc(cfg(feature = "form")))]
pub mod form;
#[cfg(feature = "form")]
#[cfg_attr(docsrs, doc(cfg(feature = "form")))]
pub use form::ValidForm;

#[cfg(feature = "sonic")]
#[cfg_attr(docsrs, doc(cfg(feature = "sonic")))]
pub mod sonic;
use mime::Mime;
#[cfg(feature = "sonic")]
#[cfg_attr(docsrs, doc(cfg(feature = "sonic")))]
pub use sonic::ValidSonic;

pub mod rejection;

fn check_content_type(headers: &HeaderMap, expected_content_type: Mime) -> bool {
    let content_type = if let Some(content_type) = headers.get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<Mime>() {
        mime
    } else {
        return false;
    };

    mime == expected_content_type
}
