use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Ident, Meta, Token, Type};

use super::{FieldCommand, FieldHandler};
use crate::syntax::field::FieldName;

pub struct ForwardAttr;

impl FieldCommand for ForwardAttr {
    fn ident(&self) -> &'static str {
        "forward_attr"
    }

    fn parse(&self, _ty: &Type, meta: Meta) -> syn::Result<Box<dyn FieldHandler>> {
        let lst = meta.require_list()?;
        let out =
            lst.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        Ok(Box::new(ForwardAttrHandler { attrs: out }))
    }
}

pub struct ForwardAttrHandler {
    attrs: Punctuated<Meta, Token![,]>,
}

impl FieldHandler for ForwardAttrHandler {
    fn in_type(&self) -> Option<Type> {
        None
    }

    fn out_type(&self) -> Option<Type> {
        None
    }

    fn gen_expr(&self, _err: &Ident, _field: &FieldName) -> syn::Result<TokenStream> {
        Ok(Default::default())
    }

    fn gen_raw_attr(&self, _field: &FieldName) -> Option<TokenStream> {
        let attr = self.attrs.iter();
        Some(quote! {
            #(
                #[#attr]
            )*
        })
    }
}
