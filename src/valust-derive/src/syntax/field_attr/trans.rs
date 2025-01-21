use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::spanned::Spanned;
use syn::{Ident, Meta, Type};

use super::{FieldCommand, FieldHandler};
use crate::cmd::trans::{TRANS_COMMANDS, TransHandler};
use crate::syntax::field::FieldName;
use crate::utils::create_error::create_transform_error;
use crate::utils::error::SyntaxError;

const META_SYNTAX_ERR: &str = "\
    Invalid `trans` usage.\n\
    For usages, refer to the crate's doc.";

pub struct Trans;

impl FieldCommand for Trans {
    fn ident(&self) -> &'static str {
        "trans"
    }

    fn parse(&self, ty: &Type, meta: Meta) -> syn::Result<Box<dyn FieldHandler>> {
        let Meta::List(lst) = meta else {
            return Err(syn::Error::new(meta.path().span(), META_SYNTAX_ERR));
        };

        let mut handlers = Vec::new();
        let mut error = SyntaxError::new();
        let out = lst.parse_nested_meta(|meta| {
            if let Some(cmd) = TRANS_COMMANDS
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
                        "Unknown `trans` command name `{}`.",
                        meta.path.to_token_stream()
                    ),
                ))
            }
        });
        if let Err(e) = out {
            error.push(e);
        }

        error.check()?;
        Ok(Box::new(TransCmdHandler {
            ty: ty.clone(),
            handlers,
        }))
    }
}

pub struct TransCmdHandler {
    ty: Type,
    handlers: Vec<Box<dyn TransHandler>>,
}

impl FieldHandler for TransCmdHandler {
    fn in_type(&self) -> Option<Type> {
        Some(
            self.handlers
                .iter()
                .find_map(|handler| handler.in_type())
                .unwrap_or(self.ty.clone()),
        )
    }

    fn out_type(&self) -> Option<Type> {
        Some(
            self.handlers
                .iter()
                .filter_map(|handler| handler.out_type())
                .next_back()
                .unwrap_or(self.ty.clone()),
        )
    }

    fn gen_expr(&self, err: &Ident, field: &FieldName) -> syn::Result<TokenStream> {
        let (_, expanded) = self.handlers.iter().fold(
            (None, TokenStream::new()),
            |(prev_ty, mut acc), h| {
                let in_ty = h.in_type();
                let out_ty = h.out_type();
                let ident = field.name();
                let expr = h.gen_transformer_expr(&ident);
                let msg = h.message(&ident);

                let decl = if let Some(ty) = &prev_ty {
                    quote! { let #ident: #ty }
                } else {
                    quote! { let #ident }
                };

                let expanded = if h.is_fallible() {
                    let cause = format_ident!(
                        "valust_trans_err_cause",
                        span = in_ty
                            .as_ref()
                            .or(prev_ty.as_ref())
                            .unwrap_or(&self.ty)
                            .span()
                    );
                    let (pre_trans, trans_fmt) = create_transform_error(
                        err,
                        field,
                        &cause,
                        msg,
                        &expr,
                        (prev_ty.as_ref().or(in_ty.as_ref()), out_ty.as_ref()),
                        true,
                    );

                    let pre_trans = if let Some(pre) = pre_trans {
                        quote! { let #pre = #ident.clone(); }
                    } else {
                        quote! {}
                    };

                    quote! {{
                        #pre_trans
                        match (#expr) {
                            ::std::result::Result::Ok(valust_v) => valust_v,
                            ::std::result::Result::Err(#cause) => {
                                #trans_fmt;
                                return None;
                            }
                        }
                    }}
                } else {
                    quote! { (#expr) }
                };

                acc.extend(quote! { #decl = #expanded; });

                (out_ty, acc)
            },
        );

        Ok(expanded)
    }
}
