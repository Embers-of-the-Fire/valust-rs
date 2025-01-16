use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, LitStr, Token, parenthesized, parse_quote};

use super::regex_comp::CompatibleExpr;

pub enum Expression {
    Expr(Expr),
    Func(Expr),
    #[cfg(feature = "regex")]
    Regex(LitStr),
}

impl Parse for Expression {
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

impl Expression {
    pub fn get_expr(&self, ident: &Ident, by_ref: bool) -> CompatibleExpr {
        match self {
            Self::Expr(expr) => {
                let p: Expr = parse_quote! { #expr };
                p.into()
            }
            Self::Func(func) => {
                let p: Expr = if by_ref {
                    parse_quote! { #func(&#ident) }
                } else {
                    parse_quote! { #func(#ident) }
                };
                p.into()
            }
            #[cfg(feature = "regex")]
            Self::Regex(lit) => CompatibleExpr::Regex(lit.clone(), ident.clone()),
        }
    }
}
