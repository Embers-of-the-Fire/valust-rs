use proc_macro2::TokenStream;
use syn::{Ident, Meta, Type};

use super::field::FieldName;

mod forward;
mod trans;
mod valid;

pub const FIELD_ATTRS: &[&dyn FieldCommand] =
    &[&forward::Forward, &valid::Valid, &trans::Trans];

pub trait FieldCommand {
    fn ident(&self) -> &'static str;

    fn parse(&self, ty: &Type, meta: Meta) -> syn::Result<Box<dyn FieldHandler>>;
}

pub trait FieldHandler {
    fn in_type(&self) -> Option<Type>;

    fn out_type(&self) -> Option<Type>;

    fn gen_expr(&self, err: &Ident, field: &FieldName) -> syn::Result<TokenStream>;
}
