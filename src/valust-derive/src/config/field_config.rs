use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{Expr, Ident, Index, Type, Visibility};

use crate::utils::create_error::{create_transform_error, create_validate_error};
use crate::utils::parser::ExprOrFunc;

pub enum FieldOperationType {
    Validate,
    Transform { from_ty: Type },
}

pub struct FieldManualOperation {
    pub ty: FieldOperationType,
    pub expr: ExprOrFunc,
    pub message: Option<Expr>,
    pub fallible: bool,
}

impl FieldManualOperation {
    pub fn get_origin_ty(&self) -> Option<Type> {
        match &self.ty {
            FieldOperationType::Transform { from_ty } => Some(from_ty.clone()),
            _ => None,
        }
    }

    pub fn to_process_op(
        &self,
        field_name: &FieldName,
        out_type: &Type,
        error_ident: &Ident,
        display: bool,
    ) -> TokenStream {
        let field = field_name.to_ident();
        let expr = self
            .expr
            .get_expr(&field, matches!(self.ty, FieldOperationType::Validate));

        let err_ident = format_ident!(
            "__valust_err_{}",
            field_name.to_ident(),
            span = field_name.get_span()
        );

        match &self.ty {
            FieldOperationType::Validate => {
                let code_invalid = create_validate_error(
                    error_ident,
                    field_name,
                    None,
                    self.message.as_ref(),
                    &expr,
                    out_type,
                    display,
                );
                let code_error = create_validate_error(
                    error_ident,
                    field_name,
                    Some(&err_ident),
                    self.message.as_ref(),
                    &expr,
                    out_type,
                    display,
                );
                if self.fallible {
                    quote! {
                        match (#expr) {
                            Ok(true) => {},
                            Ok(false) => {
                                #code_invalid
                            },
                            Err(#err_ident) => {
                                #code_error
                            }
                        }
                    }
                } else {
                    quote! {
                        if !(#expr) {
                            #code_invalid
                        }
                    }
                }
            }
            FieldOperationType::Transform { from_ty } => {
                if self.fallible {
                    let (ident_clone, code_err) = create_transform_error(
                        error_ident,
                        field_name,
                        self.message.as_ref(),
                        &err_ident,
                        &expr,
                        (from_ty, out_type),
                        display,
                    );
                    let err_pre_format = if let Some(ident_clone) = ident_clone {
                        quote! { let #ident_clone = #field.clone(); }
                    } else {
                        quote! {}
                    };

                    quote! {
                        #err_pre_format
                        let #field = match (#expr) {
                            Ok(value) => value,
                            Err(#err_ident) => {
                                #code_err
                                return None;
                            }
                        };
                    }
                } else {
                    quote! {
                        let #field = #expr;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum FieldName {
    Named(Ident),
    Unnamed(usize, Span),
}

impl FieldName {
    pub fn to_ident(&self) -> Ident {
        match self {
            FieldName::Named(name) => name.clone(),
            FieldName::Unnamed(idx, span) => Ident::new(&format!("_{}", idx), *span),
        }
    }

    pub fn to_ident_assign(&self) -> TokenStream {
        match self {
            FieldName::Named(name) => name.clone().to_token_stream(),
            FieldName::Unnamed(idx, ..) => Index::from(*idx).to_token_stream(),
        }
    }

    pub fn get_span(&self) -> Span {
        match self {
            Self::Named(name) => name.span(),
            Self::Unnamed(_, span) => *span,
        }
    }
}

pub enum FieldOperation {
    Manual(FieldManualOperation),
    Forward,
}

impl From<FieldManualOperation> for FieldOperation {
    fn from(operation: FieldManualOperation) -> Self {
        FieldOperation::Manual(operation)
    }
}

impl FieldOperation {
    pub fn get_origin_ty(&self, trans_ty: &Type) -> Option<Type> {
        match self {
            Self::Manual(m) => m.get_origin_ty(),
            Self::Forward => Some(Type::Verbatim(
                quote! { <#trans_ty as ::valust::Validate>::Raw },
            )),
        }
    }

    pub fn to_process_op(
        &self,
        field_name: &FieldName,
        out_type: &Type,
        error_ident: &Ident,
        display: bool,
    ) -> TokenStream {
        match self {
            Self::Manual(m) => {
                m.to_process_op(field_name, out_type, error_ident, display)
            }
            Self::Forward => {
                let field = field_name.to_ident();
                let field_text = field_name.to_ident_assign().to_string();
                let err = format_ident!(
                    "__valust_err_{}",
                    field,
                    span = field_name.get_span()
                );
                quote! {
                    let #field = match (<#out_type as ::valust::Validate>::validate(#field)) {
                        Ok(value) => value,
                        Err(#err) => {
                            #error_ident.extend_error(#field_text, #err);
                            return None;
                        }
                    };
                }
            }
        }
    }
}

pub struct FieldConfig {
    pub name: FieldName,
    pub vis: Visibility,
    pub ty: Type,
    pub operations: Vec<FieldOperation>,
    pub display: bool,
}

impl FieldConfig {
    pub fn to_origin_decl(&self, is_named: bool) -> TokenStream {
        let name = self.name.to_ident();
        let vis = &self.vis;
        let ty = self.get_origin_ty();

        if is_named {
            quote! { #vis #name: #ty }
        } else {
            quote! { #vis #ty }
        }
    }

    pub fn get_origin_ty(&self) -> Type {
        self.operations
            .iter()
            .find_map(|o| o.get_origin_ty(&self.ty))
            .unwrap_or(self.ty.clone())
    }

    pub fn to_process_ops(&self) -> syn::Result<(Ident, TokenStream)> {
        let error_ident = Ident::new("_valust_error", self.name.get_span());
        let tt = self
            .operations
            .iter()
            .map(|t| t.to_process_op(&self.name, &self.ty, &error_ident, self.display));
        let func_name = Ident::new(
            &format!("_valust_process_{}", self.name.to_ident()),
            self.name.get_span(),
        );
        let field = self.name.to_ident();
        let ty = &self.ty;
        let origin_ty = &self.get_origin_ty();

        Ok((func_name.clone(), quote! {
            fn #func_name(
                #field: #origin_ty,
                #error_ident: &mut ::valust::error::ValidationError
            ) -> ::std::option::Option<#ty> {
                #(#tt)*
                ::std::option::Option::Some(#field)
            }
        }))
    }
}
