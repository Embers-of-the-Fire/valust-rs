use proc_macro2::TokenStream;
use syn::parse::ParseStream;
use syn::{Ident, Type};

mod expr;
mod func;

pub const TRANS_COMMANDS: &[&dyn TransCommand] =
    &[&expr::ExprCommand, &func::FuncCommand];

pub trait TransCommand {
    fn ident(&self) -> &'static str;

    fn parse_inner(&self, tt: ParseStream) -> syn::Result<Box<dyn TransHandler>>;
}

pub trait TransHandler {
    fn in_type(&self) -> Option<Type>;

    fn out_type(&self) -> Option<Type>;

    fn gen_transformer_expr(&self, field: &Ident) -> TokenStream;

    fn message(&self, field: &Ident) -> Option<String>;

    fn is_fallible(&self) -> bool;
}
