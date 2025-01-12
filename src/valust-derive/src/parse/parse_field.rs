use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, Field, Lit, LitBool, Meta, parse2};

use super::parse_field_transformer::TransformerAttr;
use super::parse_field_validator::ValidatorAttr;
use crate::config::field_config::{FieldConfig, FieldName, FieldOperation};
use crate::utils::iter::IteratorExt;
use crate::utils::parse_error::{
    AttrPlacement, create_invalid_attr_call_error, create_misplaced_error,
    get_attr_placement,
};

pub fn parse_field_attr(field: &Field, index: usize) -> syn::Result<FieldConfig> {
    let name = field
        .ident
        .as_ref()
        .cloned()
        .map(FieldName::Named)
        .unwrap_or(FieldName::Unnamed(index, field.span()));
    let vis = field.vis.clone();
    let ty = field.ty.clone();
    let mut display = true;
    let (operations, err): (Vec<FieldOperation>, Option<syn::Error>) = field
        .attrs
        .iter()
        .filter_map(|attr| {
            let span = attr.path().span();
            if attr.path().is_ident("valid") {
                if let Meta::List(list) = &attr.meta {
                    Some(
                        parse2(list.tokens.clone()).map(ValidatorAttr::into_operations),
                    )
                } else {
                    Some(Err(create_invalid_attr_call_error(span, "valid", &[
                        "valid(...)",
                    ])))
                }
            } else if attr.path().is_ident("trans") {
                if let Meta::List(list) = &attr.meta {
                    Some(
                        parse2(list.tokens.clone())
                            .map(TransformerAttr::into_operations),
                    )
                } else {
                    Some(Err(create_invalid_attr_call_error(span, "trans", &[
                        "trans(...)",
                    ])))
                }
            } else if attr.path().is_ident("forward") {
                match &attr.meta {
                    Meta::Path(_) => Some(Ok(vec![FieldOperation::Forward])),
                    _ => Some(Err(create_invalid_attr_call_error(span, "forward", &[
                        "forward",
                    ]))),
                }
            } else if attr.path().is_ident("display") {
                match &attr.meta {
                    Meta::List(list) => match parse2::<LitBool>(list.tokens.clone()) {
                        Ok(b) => {
                            display = b.value;
                            return None;
                        }
                        Err(e) => return Some(Err(e)),
                    },
                    Meta::NameValue(nv) => {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Bool(b), ..
                        }) = &nv.value
                        {
                            display = b.value;
                            return None;
                        }
                    }
                    _ => {}
                }
                Some(Err(create_invalid_attr_call_error(span, "display", &[
                    "display = ...",
                    "display(...)",
                ])))
            } else if matches!(
                get_attr_placement(attr.path()),
                Some(AttrPlacement::Struct)
            ) {
                Some(Err(create_misplaced_error(
                    span,
                    &attr.path().to_token_stream().to_string(),
                    AttrPlacement::Struct,
                )))
            } else {
                None
            }
        })
        .collect_result_with(|i, mut o: Vec<_>| {
            o.extend(i);
            o
        });

    if let Some(err) = err {
        Err(err)
    } else {
        Ok(FieldConfig {
            name,
            vis,
            ty,
            operations,
            display,
        })
    }
}
