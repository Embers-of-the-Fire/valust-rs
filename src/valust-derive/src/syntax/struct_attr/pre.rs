use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{Expr, Ident, LitStr, Meta, Token, Type};

use crate::utils::create_error::create_meta_validate_error;

fn parse_item(buf: ParseStream) -> syn::Result<(Expr, Option<LitStr>)> {
    if buf.peek(Paren) {
        let content;
        syn::parenthesized!(content in buf);
        let expr: Expr = content.parse()?;
        let lit: Option<LitStr> = if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
            Some(content.parse()?)
        } else {
            None
        };
        Ok((expr, lit))
    } else {
        let expr: Expr = buf.parse()?;
        Ok((expr, None))
    }
}

fn parse_items(
    buf: ParseStream,
) -> syn::Result<Punctuated<(Expr, Option<LitStr>), Token![,]>> {
    Punctuated::<_, Token![,]>::parse_terminated_with(buf, parse_item)
}

pub fn parse_pre(
    meta: &Meta,
    pre: &mut Vec<(Expr, Option<LitStr>)>,
) -> syn::Result<()> {
    let lst = meta.require_list()?;
    let args: Punctuated<(Expr, Option<LitStr>), _> =
        lst.parse_args_with(parse_items)?;
    pre.extend(args);

    Ok(())
}

pub fn gen_pre_expr<'a>(
    pre: impl Iterator<Item = &'a (Expr, Option<LitStr>)>,
    name: &Ident,
    fields: impl Iterator<Item = (Ident, Type)>,
) -> (Ident, TokenStream) {
    let err_ident = format_ident!("valust_pre_err_{}", name, span = name.span());

    let mut exprs = TokenStream::new();
    for (expr, msg) in pre {
        let invalid_err = create_meta_validate_error(
            &err_ident,
            msg.as_ref().map(|m| m.value()),
            expr,
        );
        exprs.extend(quote! {
            if !(#expr) {
                #invalid_err;
                return None;
            }
        });
    }

    let fn_name = format_ident!("valust_pre_{}", name, span = name.span());
    let fields = fields.into_iter().map(|(i, ty)| quote! { #i: &#ty });
    let decl = quote! {
        fn #fn_name (#(#fields),*, #err_ident: &mut ::valust::error::ValidationError) -> ::std::option::Option<()> {
            #exprs
            ::std::option::Option::Some(())
        }
    };

    (fn_name, decl)
}
