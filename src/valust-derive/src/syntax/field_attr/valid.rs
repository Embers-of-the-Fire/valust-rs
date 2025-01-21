use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::spanned::Spanned;
use syn::{Ident, Meta, Type};

use super::{FieldCommand, FieldHandler};
use crate::cmd::valid::{VALID_COMMANDS, ValidHandler};
use crate::syntax::field::FieldName;
use crate::utils::create_error::create_validate_error;
use crate::utils::error::SyntaxError;

const META_SYNTAX_ERR: &str = "\
    Invalid `valid` usage.\n\
    For usages, refer to the crate's doc.";

pub struct Valid;

impl FieldCommand for Valid {
    fn ident(&self) -> &'static str {
        "valid"
    }

    fn parse(&self, ty: &Type, meta: Meta) -> syn::Result<Box<dyn FieldHandler>> {
        let Meta::List(lst) = meta else {
            return Err(syn::Error::new(meta.path().span(), META_SYNTAX_ERR));
        };

        let mut handlers = Vec::new();
        let mut error = SyntaxError::new();
        let out = lst.parse_nested_meta(|meta| {
            if let Some(cmd) = VALID_COMMANDS
                .iter()
                .find(|cmd| meta.path.is_ident(cmd.ident()))
            {
                let op = cmd.parse_inner(meta.input)?;
                handlers.push(op);
                Ok(())
            } else {
                Err(syn::Error::new(
                    meta.path.span(),
                    format!(
                        "Unknown `valid` command name `{}`.",
                        meta.path.to_token_stream()
                    ),
                ))
            }
        });
        if let Err(e) = out {
            error.push(e);
        }

        error.check()?;
        Ok(Box::new(ValidCmdHandler {
            ty: ty.clone(),
            handlers,
        }))
    }
}

pub struct ValidCmdHandler {
    ty: Type,
    handlers: Vec<Box<dyn ValidHandler>>,
}

impl FieldHandler for ValidCmdHandler {
    fn in_type(&self) -> Option<Type> {
        Some(self.ty.clone())
    }

    fn out_type(&self) -> Option<Type> {
        Some(self.ty.clone())
    }

    fn gen_expr(&self, err: &Ident, field: &FieldName) -> syn::Result<TokenStream> {
        let code = self.handlers.iter().map(|t| {
            let expr = t.gen_validator_expr(&field.name());
            let msg = t.message(&field.name());
            let invalid_err = create_validate_error(
                err, field, None, msg, &expr, &self.ty, true,
            );

            if t.is_fallible() {
                let cause =
                    format_ident!("valust_valid_err_cause", span = self.ty.span());
                let fail_err = create_validate_error(
                    err,
                    field,
                    Some(&cause),
                    None,
                    &expr,
                    &self.ty,
                    true,
                );

                quote! {
                    match (#expr) {
                        ::std::result::Result::Ok(true) => {},
                        ::std::result::Result::Ok(false) => { #invalid_err; return None; },
                        ::std::result::Result::Err(#cause) => { #fail_err; return None; },
                    }
                }
            } else {
                quote! {
                    if !(#expr) {
                        #invalid_err;
                        return None;
                    }
                }
            }
        });

        Ok(quote! { #(#code)* })
    }
}
