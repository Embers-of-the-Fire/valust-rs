use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::Type;

use crate::syntax::field::FieldName;

/// Output
///
/// ```rust,ignore
/// error.push_validate_error(ValidateError { .. })
/// ```
pub fn create_validate_error(
    error_ident: &Ident,
    field: &FieldName,
    cause: Option<&Ident>,
    message: Option<String>,
    expr: impl AsRef<str>,
    ty: &Type,
    display: bool,
) -> TokenStream {
    let field_text = field.struct_key().to_string();
    let field = field.name();
    let cause = cause
        .map(|cause| quote! { ::std::option::Option::Some(::std::boxed::Box::new(#cause)) })
        .unwrap_or(quote! { ::std::option::Option::None });
    let message = message
        .map(|m| quote! { ::std::option::Option::Some(#m) })
        .unwrap_or(quote! { ::std::option::Option::None });
    let expr_text = expr.as_ref();
    let type_text = ty.to_token_stream().to_string();
    let value_format = if display {
        let value_formatter = format!("({}) {{:?}}", type_text);
        quote! { format!(#value_formatter, #field) }
    } else {
        let value_formatter = format!("({})", type_text);
        quote! { format!(#value_formatter)}
    };

    quote! {
        #error_ident.push_validate_error(
            ::valust::error::validate::ValidateError {
                field: #field_text,
                path: format!("{}", #field_text),
                value: #value_format,
                cause: #cause,
                message: #message,
                expression: #expr_text,
                type_name: #type_text,
            }
        )
    }
}

/// Output
///
/// ```rust,ignore
/// error.push_validate_error(ValidateError { .. })
/// ```
pub fn create_meta_validate_error(
    error_ident: &Ident,
    message: Option<String>,
    expr: impl ToTokens,
) -> TokenStream {
    let message = message
        .map(|m| quote! { ::std::option::Option::Some(#m) })
        .unwrap_or(quote! { ::std::option::Option::None });
    let expr_text = expr.to_token_stream().to_string();

    quote! {
        #error_ident.push_validate_error(
            ::valust::error::validate::ValidateError {
                field: "<meta>",
                path: format!("<meta>"),
                value: format!("<meta>"),
                cause: ::std::option::Option::None,
                message: #message,
                expression: #expr_text,
                type_name: "<meta>",
            }
        )
    }
}

/// Output
///
/// ```rust,ignore
/// error.push_validate_error(TransformError { .. })
/// ```
pub fn create_transform_error(
    error_ident: &Ident,
    field: &FieldName,
    cause: &Ident,
    message: Option<String>,
    expr: impl ToTokens,
    (origin_ty, out_ty): (Option<&Type>, Option<&Type>),
    display: bool,
) -> (Option<Ident>, TokenStream) {
    let field_text = field.struct_key().to_string();
    let field = field.name();
    let message = message
        .map(|m| quote! { ::std::option::Option::Some(#m) })
        .unwrap_or(quote! { ::std::option::Option::None });
    let expr_text = expr.to_token_stream().to_string();
    let orig_type_text = if let Some(origin_ty) = origin_ty {
        origin_ty.to_token_stream().to_string()
    } else {
        "<unknown>".to_string()
    };
    let out_type_text = if let Some(out_ty) = out_ty {
        out_ty.to_token_stream().to_string()
    } else {
        "<unknown>".to_string()
    };
    let (ident_clone, value_format) = if display {
        let value_formatter = format!("({}) {{:?}}", orig_type_text);
        let ident_clone =
            format_ident!("valust_format_err_clone_{}", field, span = field.span());
        (
            Some(ident_clone.clone()),
            quote! { format!(#value_formatter, #ident_clone) },
        )
    } else {
        let value_formatter = format!("({})", orig_type_text);
        (None, quote! { format!(#value_formatter) })
    };

    let expanded = quote! {
        #error_ident.push_transform_error(
            ::valust::error::transform::TransformError {
                field: #field_text,
                path: format!("{}", #field_text),
                value: #value_format,
                cause: ::std::boxed::Box::new(#cause),
                message: #message,
                expression: #expr_text,
                source_type_name: #orig_type_text,
                target_type_name: #out_type_text,
            }
        );
    };
    (ident_clone, expanded)
}
