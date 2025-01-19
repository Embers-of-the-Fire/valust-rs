//! Derive macro implementation for `Valust`.
#![doc = include_str!("../README.md")]

// mod config;
// mod parse;
mod cmd;
mod syntax;
mod utils;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use syntax::structure::Structure;

// const FIELD_ATTRS: &[&str] = &["valid", "trans", "forward", "display"];
// const STRUCT_ATTRS: &[&str] = &["pre", "post", "rename", "forward_derive"];

/// Main entry point for the `Valust` macro.
///
/// For full documentation, see the crates's README file.
#[proc_macro_derive(
    Valust,
    attributes(valid, trans, forward, pre, post, rename, forward_derive, display)
)]
pub fn valust_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let data = match Structure::from_input(input) {
        Ok(t) => t,
        Err(e) => return e.to_compile_error().into(),
    };
    let expanded = match data.gen_validate_impl() {
        Ok(t) => t,
        Err(e) => return e.to_compile_error().into(),
    };

    expanded.into()
}
