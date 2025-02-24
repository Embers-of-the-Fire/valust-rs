use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Visibility};

use super::field::Field;
use super::struct_attr::StructAttr;
use crate::utils::error::SyntaxError;

const UNSUPPORTED_STRUCT_TYPE: &str = "\
    Unsupported struct type. \
    Valust only supports struct with named fields or unnamed fields.";

pub struct Structure {
    pub vis: Visibility,
    pub name: Ident,
    pub is_named: bool,
    pub fields: Vec<Field>,
    pub attrs: StructAttr,
}

impl Structure {
    pub fn from_input(s: DeriveInput) -> syn::Result<Self> {
        let (is_named, fields) = match s.data {
            Data::Struct(st) => match st.fields {
                Fields::Named(n) => (true, n.named),
                Fields::Unnamed(u) => (false, u.unnamed),
                Fields::Unit => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        UNSUPPORTED_STRUCT_TYPE,
                    ));
                }
            },
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    UNSUPPORTED_STRUCT_TYPE,
                ));
            }
        };

        Ok(Self {
            vis: s.vis,
            name: s.ident,
            is_named,
            fields: {
                let mut out = Vec::with_capacity(fields.len());
                let mut err = SyntaxError::new();
                for (idx, f) in fields.into_iter().enumerate() {
                    match Field::from_input(f, idx) {
                        Ok(field) => out.push(field),
                        Err(e) => err.push(e),
                    }
                }
                err.check()?;
                out
            },
            attrs: StructAttr::from_attrs(s.attrs.iter())?,
        })
    }

    pub fn gen_validate_impl(&self) -> syn::Result<TokenStream> {
        let names = self.fields.iter().map(|t| t.name.clone());
        let ty = &self.name;
        let st_vis = &self.vis;

        let mut error = SyntaxError::new();

        let raw_name = self.attrs.rename.clone().unwrap_or_else(|| {
            format_ident!("Raw{}", self.name, span = self.name.span())
        });
        let raw_decl = {
            let decls = self.fields.iter().map(|t| {
                let ty = t.get_raw_type();
                let vis = &t.vis;
                let attr = t.operations.iter().flat_map(|op| op.gen_raw_attr(&t.name));
                if self.is_named {
                    let name = t.name.name();
                    quote! {
                        #(#attr)*
                        #vis #name: #ty
                    }
                } else {
                    quote! {
                        #(#attr)*
                        #vis #ty
                    }
                }
            });
            let derives = &self.attrs.forward_derive;
            let attrs = &self.attrs.forward_attr;
            let decl = if self.is_named {
                quote! { { #(#decls,)* } }
            } else {
                quote! { (#(#decls),*); }
            };
            quote! {
                #[derive(#(#derives),*)]
                #( #[#attrs] )*
                #st_vis struct #raw_name #decl
            }
        };

        let packing_names = names.clone().map(|name| name.name());
        let unpack_raw = if self.is_named {
            quote! { let #raw_name{ #(#packing_names,)* } = raw; }
        } else {
            quote! { let #raw_name(#(#packing_names),*) = raw; }
        };

        let err_ident =
            format_ident!("valust_impl_err_{}", self.name, span = self.name.span());
        let err_init = quote! {
            let mut #err_ident = ::valust::error::ValidationError::new();
        };

        let (pre, post) = {
            let fields = self.fields.iter().map(|f| f.name.name());

            let pre = if !self.attrs.pre.is_empty() {
                let (pre, pre_func) = self.attrs.gen_pre_expr(
                    &self.name,
                    self.fields
                        .iter()
                        .map(|field| (field.name.name(), field.get_raw_type())),
                );
                let pre_fields = fields.clone();
                quote! {
                    #pre_func
                    #pre(#(&#pre_fields),*, &mut #err_ident);
                }
            } else {
                quote! {}
            };

            let post = if !self.attrs.post.is_empty() {
                let (post, post_func) = self.attrs.gen_post_expr(
                    &self.name,
                    self.fields
                        .iter()
                        .map(|field| (field.name.name(), &field.ty)),
                );
                let post_fields = fields;
                quote! {
                    #post_func
                    #post(#(&#post_fields),*, &mut #err_ident);
                }
            } else {
                quote! {}
            };
            (pre, post)
        };

        let field_validate = self
            .fields
            .iter()
            .map(|field| -> syn::Result<TokenStream> {
                let ident = field.name.name();
                let out_ty = &field.ty;
                let (func_name, func_body) = field.gen_validate_func()?;
                Ok(quote! {
                    #func_body
                    let #ident: Option<#out_ty> = #func_name(#ident, &mut #err_ident);
                })
            })
            .filter_map(|field| match field {
                Ok(field) => Some(field),
                Err(err) => {
                    error.push(err);
                    None
                }
            });

        let unwrap_validated = {
            let check = names.clone().map(|name| {
                let text = format!(
                    "Unexpected error occurred in processing field `{}`",
                    name.struct_key()
                );
                let name = name.name();
                quote! { let #name = #name.expect(#text); }
            });
            quote! { #(#check)* }
        };

        let packing_names = names.clone().map(|name| name.name());
        let pack_raw = if self.is_named {
            quote! { #ty{ #(#packing_names,)* } }
        } else {
            quote! { #ty(#(#packing_names),*) }
        };

        let expanded = quote! {
            #[automatically_derived]
            #raw_decl

            #[automatically_derived]
            #[allow(non_camel_case_types, non_snake_case, unused_variables, non_upper_case_globals)]
            impl ::valust::Validate for #ty {
                type Raw = #raw_name;

                fn validate(raw: Self::Raw) -> Result<Self, ::valust::error::ValidationError> {
                    #unpack_raw

                    #err_init
                    #pre
                    #err_ident.check()?;

                    #err_init
                    #(#field_validate)*
                    #err_ident.check()?;

                    #unwrap_validated

                    #err_init
                    #post
                    #err_ident.check()?;

                    Ok(#pack_raw)
                }
            }
        };
        error.check()?;
        Ok(expanded)
    }
}
