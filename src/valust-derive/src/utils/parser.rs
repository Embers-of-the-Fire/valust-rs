use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token, parenthesized, parse_quote};

pub enum ExprOrFunc {
    Expr(Expr),
    Func(Expr),
}

impl Parse for ExprOrFunc {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![fn]) {
            input.parse::<Token![fn]>()?;
            let content;
            let _ = parenthesized!(content in input);
            Ok(Self::Func(content.parse()?))
        } else {
            Ok(Self::Expr(input.parse()?))
        }
    }
}

impl ExprOrFunc {
    pub fn get_expr(&self, ident: &Ident, by_ref: bool) -> Expr {
        match self {
            Self::Expr(expr) => parse_quote! { #expr },
            Self::Func(func) => {
                if by_ref {
                    parse_quote! { #func(&#ident) }
                } else {
                    parse_quote! { #func(#ident) }
                }
            }
        }
    }
}
