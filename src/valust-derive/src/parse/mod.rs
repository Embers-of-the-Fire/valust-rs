pub mod parse_field;
pub mod parse_field_transformer;
pub mod parse_field_validator;
pub mod parse_struct;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Result};

use crate::parse::parse_struct::parse_struct;

pub fn parse_input(input: DeriveInput) -> Result<TokenStream> {
    parse_struct(input)?.to_trait_impl()
}
