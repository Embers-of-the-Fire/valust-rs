use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{Expr, Ident, LitStr, Token};

use super::{ValidCommand, ValidHandler};

pub struct RegexCommand;

impl ValidCommand for RegexCommand {
    fn ident(&self) -> &'static str {
        "regex"
    }

    fn parse_inner(&self, tt: ParseStream) -> syn::Result<Box<dyn ValidHandler>> {
        let content;
        syn::parenthesized!(content in tt);
        let regex = content.parse::<Expr>()?;
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
            if !content.is_empty() {
                let message = Some(content.parse()?);
                return Ok(Box::new(RegexHandler { regex, message }));
            }
        }
        Ok(Box::new(RegexHandler {
            regex,
            message: None,
        }))
    }
}

struct RegexHandler {
    regex: Expr,
    message: Option<LitStr>,
}

impl ValidHandler for RegexHandler {
    fn gen_validator_expr(&self, field: &Ident) -> TokenStream {
        let regex = &self.regex;
        let regex_name =
            format_ident!("valust_valid_regex_{}", field, span = self.regex.span());

        quote! {{
            static #regex_name: ::std::sync::LazyLock<::valust::regex::Regex> = ::std::sync::LazyLock::new(|| {
                ::valust::regex::Regex::new(#regex).unwrap()
            });
            #regex_name.is_match(&#field)
        }}
    }

    fn message(&self, field: &Ident) -> Option<String> {
        Some(self.message.as_ref().map_or_else(
            || format!("`{}` does not match the regex", field),
            |lit| lit.value(),
        ))
    }

    fn gen_expr_display(&self, _field: &Ident) -> Option<String> {
        Some(format!("<regex>/{}/", self.regex.to_token_stream()))
    }

    fn is_fallible(&self) -> bool {
        false
    }
}
