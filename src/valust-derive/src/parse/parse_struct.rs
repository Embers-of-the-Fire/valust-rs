use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, Fields, Token};

use crate::config::struct_config::{StructAttr, StructConfig};
use crate::parse::parse_field::parse_field_attr;
use crate::parse::parse_field_validator::ValidatorItem;
use crate::utils::iter::IteratorExt;
use crate::utils::option::OptionExt;

pub struct StructForwardDerive {
    pub derives: Punctuated<Ident, Token![,]>,
}

impl Parse for StructForwardDerive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            derives: input.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

pub struct StructPreValidatorAttr {
    pub pre: Punctuated<ValidatorItem, Token![,]>,
}

impl Parse for StructPreValidatorAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pre: input.parse_terminated(ValidatorItem::parse, Token![,])?,
        })
    }
}

pub struct StructPostValidatorAttr {
    pub post: Punctuated<ValidatorItem, Token![,]>,
}

impl Parse for StructPostValidatorAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            post: input.parse_terminated(ValidatorItem::parse, Token![,])?,
        })
    }
}

pub fn parse_struct(input: DeriveInput) -> syn::Result<StructConfig> {
    if let Data::Struct(st) = input.data {
        let (fields, err) = st
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| parse_field_attr(f, i))
            .collect_result();
        err.err_or(())?;

        let validator = StructAttr::from_attrs(input.attrs.iter())?;

        Ok(StructConfig {
            name: input.ident,
            vis: input.vis,
            fields,
            attrs: validator,
            is_named: matches!(st.fields, Fields::Named(_)),
        })
    } else {
        Err(syn::Error::new(
            Span::call_site(),
            "`Valust` can only be derived for structs",
        ))
    }
}
