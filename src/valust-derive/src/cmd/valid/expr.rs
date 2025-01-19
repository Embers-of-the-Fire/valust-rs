use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::ParseStream;
use syn::{Expr, Ident, LitStr, Token};

use super::{ValidCommand, ValidHandler};

pub struct ExprCommand;

impl ValidCommand for ExprCommand {
    fn ident(&self) -> &'static str {
        "expr"
    }

    fn parse_inner(&self, tt: ParseStream) -> syn::Result<Box<dyn ValidHandler>> {
        let content;
        syn::parenthesized!(content in tt);
        let (expr, fallible): (Expr, _) = if content.peek(Token![try]) {
            content.parse::<Token![try]>()?;
            let try_content;
            syn::parenthesized!(try_content in content);
            (try_content.parse()?, true)
        } else {
            (content.parse()?, false)
        };
        let message: Option<LitStr> = if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
            Some(content.parse()?)
        } else {
            None
        };

        Ok(Box::new(ExprHandler {
            expr,
            fallible,
            message,
        }))
    }
}

pub struct ExprHandler {
    expr: Expr,
    fallible: bool,
    message: Option<LitStr>,
}

impl ValidHandler for ExprHandler {
    fn gen_validator_expr(&self, _field: &Ident) -> TokenStream {
        let expr = &self.expr;
        quote! { { #expr } }
    }

    fn is_fallible(&self) -> bool {
        self.fallible
    }

    fn message(&self, field: &Ident) -> Option<String> {
        Some(self.message.as_ref().map_or_else(
            || format!("`{}`'s validator expression evaluate to `false`", field),
            |lit| lit.value(),
        ))
    }
}
