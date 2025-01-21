use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::ParseStream;
use syn::{Expr, Ident, LitStr, Token, Type};

use super::{TransCommand, TransHandler};

pub struct FuncCommand;

impl TransCommand for FuncCommand {
    fn ident(&self) -> &'static str {
        "func"
    }

    fn parse_inner(&self, tt: ParseStream) -> syn::Result<Box<dyn TransHandler>> {
        let content;
        syn::parenthesized!(content in tt);
        let in_type = {
            let fork = content.fork();
            if let Ok(ty) = fork.parse::<Type>() {
                if fork.peek(Token![=>]) {
                    content.parse::<Type>()?;
                    content.parse::<Token![=>]>()?;
                    Some(ty)
                } else {
                    None
                }
            } else {
                None
            }
        };
        let (expr, fallible): (Expr, _) = {
            if content.peek(Token![try]) {
                content.parse::<Token![try]>()?;
                let expr;
                syn::parenthesized!(expr in content);
                (expr.parse()?, true)
            } else {
                (content.parse()?, false)
            }
        };
        let out_type = if content.peek(Token![=>]) {
            content.parse::<Token![=>]>()?;
            Some(content.parse()?)
        } else {
            None
        };
        let message = if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
            Some(content.parse()?)
        } else {
            None
        };

        Ok(Box::new(FuncHandler {
            in_type,
            expr,
            fallible,
            message,
            out_type,
        }))
    }
}

pub struct FuncHandler {
    in_type: Option<Type>,
    expr: Expr,
    fallible: bool,
    message: Option<LitStr>,
    out_type: Option<Type>,
}

impl TransHandler for FuncHandler {
    fn in_type(&self) -> Option<Type> {
        self.in_type.clone()
    }

    fn out_type(&self) -> Option<Type> {
        self.out_type.clone()
    }

    fn is_fallible(&self) -> bool {
        self.fallible
    }

    fn message(&self, field: &Ident) -> Option<String> {
        Some(self.message.as_ref().map_or_else(
            || format!("`{}`'s transform expression fails", field),
            |lit| lit.value(),
        ))
    }

    fn gen_transformer_expr(&self, field: &Ident) -> TokenStream {
        let func = &self.expr;
        quote! { (#func)(#field) }
    }
}
