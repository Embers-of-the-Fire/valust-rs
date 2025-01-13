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
#[cfg(feature = "sonic")]
#[cfg_attr(docsrs, doc(cfg(feature = "sonic")))]
pub use sonic::ValidSonic;

#[cfg(feature = "yaml")]
#[cfg_attr(docsrs, doc(cfg(feature = "yaml")))]
pub mod yaml;
#[cfg(feature = "yaml")]
#[cfg_attr(docsrs, doc(cfg(feature = "yaml")))]
pub use yaml::ValidYaml;

#[cfg(feature = "toml")]
#[cfg_attr(docsrs, doc(cfg(feature = "toml")))]
pub mod toml;
#[cfg(feature = "toml")]
#[cfg_attr(docsrs, doc(cfg(feature = "toml")))]
pub use toml::ValidToml;

use mime::Mime;

pub mod rejection;

fn check_content_type(headers: &HeaderMap, expected_content_type: &'static str) -> bool {
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
