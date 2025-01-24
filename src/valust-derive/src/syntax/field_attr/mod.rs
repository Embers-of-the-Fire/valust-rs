use proc_macro2::TokenStream;
use syn::{Ident, Meta, Type};

use super::field::FieldName;

mod forward;
mod forward_attr;
mod trans;
mod valid;

pub const FIELD_ATTRS: &[&dyn FieldCommand] = &[
    &forward::Forward,
    &valid::Valid,
    &trans::Trans,
    &forward_attr::ForwardAttr,
];

pub trait FieldCommand {
    fn ident(&self) -> &'static str;

    fn parse(&self, ty: &Type, meta: Meta) -> syn::Result<Box<dyn FieldHandler>>;
}

pub trait FieldHandler {
    fn in_type(&self) -> Option<Type>;

    fn out_type(&self) -> Option<Type>;

    fn gen_expr(&self, err: &Ident, field: &FieldName) -> syn::Result<TokenStream>;

    // generate attr over raw item.
    // with `#[]` wrapper
    fn gen_raw_attr(&self, _field: &FieldName) -> Option<TokenStream> {
        None
    }
}
