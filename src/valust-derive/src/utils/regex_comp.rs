use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Expr, Ident, LitStr};

pub enum CompatibleExpr {
    Expr(Expr),
    #[cfg(feature = "regex")]
    #[allow(dead_code)]
    Regex(LitStr, Ident),
}

impl CompatibleExpr {
    pub fn as_expr(&self) -> TokenStream {
        match self {
            Self::Expr(e) => e.to_token_stream(),
            Self::Regex(text, input) => {
                let lock_init = quote! {
                    ::std::sync::LazyLock::new(|| {
                        // we use `unwrap` here because we can do nothing if
                        // the regex is invalid.
                        ::valust::regex::Regex::new(#text).unwrap()
                    })
                };
                let lock_name = format_ident!("__valust_regex");
                quote! {{
                    #[allow(non_upper_case_globals)]
                    static #lock_name: ::std::sync::LazyLock<::valust::regex::Regex> = #lock_init;
                    #lock_name.is_match(&#input)
                }}
            }
        }
    }

    pub fn to_expr_text(&self) -> String {
        match self {
            Self::Expr(e) => e.to_token_stream().to_string(),
            #[cfg(feature = "regex")]
            Self::Regex(rg, _) => format!("<regex>/{}/", rg.value()),
        }
    }
}

impl From<Expr> for CompatibleExpr {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}
