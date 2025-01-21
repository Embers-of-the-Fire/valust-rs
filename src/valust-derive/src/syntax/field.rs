use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Ident, Type, Visibility};

use super::field_attr::{FIELD_ATTRS, FieldHandler};
use crate::utils::error::SyntaxError;

#[derive(Debug, Clone)]
pub enum FieldName {
    Named(Ident),
    UnNamed(usize, Span),
}

impl FieldName {
    pub fn name(&self) -> Ident {
        match self {
            FieldName::Named(name) => name.clone(),
            FieldName::UnNamed(idx, span) => {
                format_ident!("_{}", idx.to_string(), span = *span)
            }
        }
    }

    pub fn span(&self) -> Span {
        match self {
            FieldName::Named(name) => name.span(),
            FieldName::UnNamed(_, span) => *span,
        }
    }

    pub fn struct_key(&self) -> TokenStream {
        match self {
            FieldName::Named(name) => quote_spanned! { name.span() => #name },
            FieldName::UnNamed(idx, span) => {
                let key = Literal::usize_unsuffixed(*idx);
                quote_spanned! { *span => #key }
            }
        }
    }
}

pub struct Field {
    pub vis: Visibility,
    pub name: FieldName,
    pub ty: Type,
    pub operations: Vec<Box<dyn FieldHandler>>,
}

impl Field {
    pub fn from_input(s: syn::Field, index: usize) -> syn::Result<Self> {
        Ok(Self {
            operations: {
                let mut attrs = Vec::new();
                let mut err = SyntaxError::new();
                for attr in s.attrs {
                    if let Some(cmd) = FIELD_ATTRS
                        .iter()
                        .find(|cmd| attr.path().is_ident(cmd.ident()))
                    {
                        let handler = cmd.parse(&s.ty, attr.meta);
                        match handler {
                            Ok(h) => attrs.push(h),
                            Err(e) => err.push(e),
                        }
                    }
                }
                err.check()?;
                attrs
            },
            vis: s.vis,
            name: s
                .ident
                .map_or(FieldName::UnNamed(index, s.ty.span()), FieldName::Named),
            ty: s.ty,
        })
    }

    fn infer_in_type(&self) -> Option<Type> {
        self.operations.iter().find_map(|op| op.in_type())
    }

    fn infer_out_type(&self) -> Option<Type> {
        self.operations
            .iter()
            .filter_map(|op| op.out_type())
            .next_back()
    }

    pub fn get_raw_type(&self) -> Type {
        self.infer_in_type().unwrap_or(self.ty.clone())
    }

    pub fn gen_validate_func(&self) -> syn::Result<(Ident, TokenStream)> {
        let in_type = self.infer_in_type().unwrap_or(self.ty.clone());
        let out_type = self.infer_out_type().unwrap_or(self.ty.clone());
        let field_ident = &self.name.name();

        let err_ident = format_ident!(
            "valust_err_{}",
            self.name.struct_key().to_string(),
            span = self.name.span()
        );

        let mut error = SyntaxError::new();

        let block = self
            .operations
            .iter()
            .map(|item| item.gen_expr(&err_ident, &self.name))
            .filter_map(|code| match code {
                Ok(code) => Some(code),
                Err(err) => {
                    error.push(err);
                    None
                }
            });

        let func_name = format_ident!(
            "valust_validate_{}",
            self.name.struct_key().to_string(),
            span = self.name.span()
        );

        let expanded = quote! {
            fn #func_name(#field_ident: #in_type, #err_ident: &mut ::valust::error::ValidationError) -> Option<#out_type> {
                #(#block)*
                Some(#field_ident)
            }
        };

        error.check().map(|_| (func_name, expanded))
    }
}
