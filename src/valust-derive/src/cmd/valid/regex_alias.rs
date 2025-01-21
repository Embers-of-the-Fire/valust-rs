use paste::paste;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;
use syn::parse::ParseStream;

use super::{ValidCommand, ValidHandler};

macro_rules! __valust_regex_alias {
    ($ident:ident feature($feat:literal) fmt($fmt:literal) = $reg:expr) => {
        paste! {
            pub struct [< $ident:camel Command >];

            impl ValidCommand for [< $ident:camel Command >] {
                fn ident(&self) -> &'static str {
                    stringify!($ident)
                }

                fn parse_inner(&self, _tt: ParseStream) -> syn::Result<Box<dyn ValidHandler>> {
                    #[cfg(feature = $feat)]
                    return Ok(Box::new([< $ident:camel Handler >]));
                    #[cfg(not(feature = $feat))]
                    return Err(syn::Error::new(proc_macro2::Span::call_site(), concat!("feature `", $feat, "` is not enabled")));
                }
            }

            #[allow(dead_code)]
            struct [< $ident:camel Handler >];

            impl ValidHandler for [< $ident:camel Handler >] {
                fn is_fallible(&self) -> bool {
                    false
                }

                fn message(&self, field: &Ident) -> Option<String> {
                    Some(format!($fmt, field))
                }

                fn gen_expr_display(&self, _field: &Ident) -> Option<String> {
                    Some(format!("<regex>/{}/", $reg))
                }

                fn gen_validator_expr(&self, field: &Ident) -> TokenStream {
                    let regex = $reg;
                    let regex_name =
                        format_ident!(concat!("valust_valid_", stringify!($ident), "_{}"), field, span = field.span());

                    quote! {{
                        static #regex_name: ::std::sync::LazyLock<::valust::regex::Regex> = ::std::sync::LazyLock::new(|| {
                            ::valust::regex::Regex::new(#regex).unwrap()
                        });
                        #regex_name.is_match(&#field)
                    }}
                }
            }
        }
    };
}

__valust_regex_alias!(
    email
        feature("email")
        fmt("`{}` is not a valid email address")
    = valust_regex_utils::EMAIL
);

__valust_regex_alias!(
    url
        feature("url")
        fmt("`{}` is not a valid URL")
    = valust_regex_utils::URL
);

__valust_regex_alias!(
    username
        feature("username")
        fmt("`{}` is not a valid username")
    = valust_regex_utils::USERNAME
);
