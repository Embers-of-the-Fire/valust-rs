use proc_macro2::Span;
use syn::Path;

use crate::{FIELD_ATTRS, STRUCT_ATTRS};

#[derive(Debug, Clone, Copy)]
pub enum AttrPlacement {
    Struct,
    Field,
}

pub fn create_misplaced_error(
    span: Span,
    key: &str,
    placement: AttrPlacement,
) -> syn::Error {
    syn::Error::new(
        span,
        format!(
            "Invalid attribute placement, attribute `{}` should be placed on {}",
            key,
            match placement {
                AttrPlacement::Field => "a field",
                AttrPlacement::Struct => "the struct",
            }
        ),
    )
}

pub fn create_invalid_attr_call_error(
    span: Span,
    key: &str,
    valid_forms: &'static [&'static str],
) -> syn::Error {
    syn::Error::new(
        span,
        format!(
            "Invalid attribute call, attribute `{}` should be called like {}",
            key,
            valid_forms
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(" or ")
        ),
    )
}

pub fn get_attr_placement(path: &Path) -> Option<AttrPlacement> {
    if FIELD_ATTRS.iter().any(|t| path.is_ident(t)) {
        Some(AttrPlacement::Field)
    } else if STRUCT_ATTRS.iter().any(|t| path.is_ident(t)) {
        Some(AttrPlacement::Struct)
    } else {
        None
    }
}
