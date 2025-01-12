//! Derive macro implementation for `Valust`.
//!
//! For full documentation, see the README file.

mod config;
mod parse;
mod utils;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

const FIELD_ATTRS: &[&str] = &["valid", "trans", "forward", "display"];
const STRUCT_ATTRS: &[&str] = &["pre", "post", "rename", "forward_derive"];

fn debug(_name: &str, tt: TokenStream) -> TokenStream {
    std::fs::write(
        format!(
            "{}/test.out.{}.rs",
            std::env::var("CARGO_MANIFEST_DIR").unwrap(),
            _name
        ),
        tt.to_string(),
    )
    .unwrap();
    tt
}

/// Main entry point for the `Valust` macro.
///
/// For full documentation, see the crates's README file.
#[proc_macro_derive(
    Valust,
    attributes(valid, trans, forward, pre, post, rename, forward_derive, display)
)]
pub fn valust_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let input_name = input.ident.to_string();

    let expanded = parse::parse_input(input);

    match expanded {
        Ok(e) => debug(input_name.as_str(), TokenStream::from(e)),
        Err(e) => e.to_compile_error().into(),
    }
}
