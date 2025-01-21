use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Ident, Meta, Type, parse_quote};

use super::{FieldCommand, FieldHandler};
use crate::syntax::field::FieldName;

const META_SYNTAX_ERR: &str = "\
    Invalid `forward` usage.\n\
    Expect to be `#[forward]` or `#[forward(Type)]`.";

pub struct Forward;

impl FieldCommand for Forward {
    fn ident(&self) -> &'static str {
        "forward"
    }

    fn parse(&self, ty: &Type, meta: Meta) -> syn::Result<Box<dyn FieldHandler>> {
        match meta {
            Meta::Path(_) => Ok(Box::new(ForwardCmdHandler { ty: ty.clone() })),
            Meta::List(lst) => Ok(Box::new(ForwardCmdHandler {
                ty: lst.parse_args()?,
            })),
            _ => Err(syn::Error::new(meta.path().span(), META_SYNTAX_ERR)),
        }
    }
}

pub struct ForwardCmdHandler {
    ty: Type,
}

impl FieldHandler for ForwardCmdHandler {
    fn in_type(&self) -> Option<syn::Type> {
        let ty = &self.ty;
        Some(parse_quote! { ::valust::Raw::<#ty> })
    }

    fn out_type(&self) -> Option<syn::Type> {
        Some(self.ty.clone())
    }

    fn gen_expr(&self, err: &Ident, field: &FieldName) -> syn::Result<TokenStream> {
        let field_ident = field.name();
        let field_text = field.struct_key().to_string();
        let out_type = &self.ty;

        Ok(quote! {
            let #field_ident: #out_type = match ::valust::Validate::validate(#field_ident) {
                Ok(v_valust) => v_valust,
                Err(e_valust) => {
                    #err.extend_error(#field_text, e_valust);
                    return None;
                },
            };
        })
    }
}
