use proc_macro2::TokenStream;
use syn::Ident;
use syn::parse::ParseStream;

mod expr;
mod func;
mod regex;

pub const VALID_COMMANDS: &[&dyn ValidCommand] =
    &[&regex::RegexCommand, &expr::ExprCommand];

pub trait ValidCommand {
    fn ident(&self) -> &'static str;

    fn parse_inner(&self, tt: ParseStream) -> syn::Result<Box<dyn ValidHandler>>;
}

pub trait ValidHandler {
    /// This should be an expression returns either a boolean or a `Result<bool, E>`.
    ///
    /// If this returns a `Result`, `is_fallible` must be `true`, vice versa.
    fn gen_validator_expr(&self, field: &Ident) -> TokenStream;

    /// Error message.
    ///
    /// This will be used like `Some(self.message())`.
    fn message(&self, field: &Ident) -> Option<String>;

    fn is_fallible(&self) -> bool;
}
