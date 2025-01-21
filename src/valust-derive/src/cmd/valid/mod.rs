use proc_macro2::TokenStream;
use syn::Ident;
use syn::parse::ParseStream;

mod color;
mod expr;
mod func;
mod regex;
#[cfg(feature = "regex-utils")]
mod regex_alias;

pub const VALID_COMMANDS: &[&dyn ValidCommand] = &[
    &regex::RegexCommand,
    &expr::ExprCommand,
    &func::FuncCommand,
    #[cfg(feature = "regex-utils")]
    &regex_alias::EmailCommand,
    #[cfg(feature = "regex-utils")]
    &regex_alias::UrlCommand,
    #[cfg(feature = "regex-utils")]
    &regex_alias::UsernameCommand,
    &color::ColorCommand,
];

pub trait ValidCommand {
    fn ident(&self) -> &'static str;

    fn parse_inner(&self, tt: ParseStream) -> syn::Result<Box<dyn ValidHandler>>;
}

pub trait ValidHandler {
    /// This should be an expression returns either a boolean or a `Result<bool, E>`.
    ///
    /// If this returns a `Result`, `is_fallible` must be `true`, vice versa.
    fn gen_validator_expr(&self, field: &Ident) -> TokenStream;

    /// Display of the inner expression. If this method returns `None`,
    /// the output display will be `self.gen_validator_expr(field).to_string()`.
    fn gen_expr_display(&self, _field: &Ident) -> Option<String> {
        None
    }

    /// Error message.
    ///
    /// This will be used like `Some(self.message())`.
    fn message(&self, field: &Ident) -> Option<String>;

    fn is_fallible(&self) -> bool;
}
