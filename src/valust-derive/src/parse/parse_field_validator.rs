use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, LitStr, Token, parenthesized, token};

use crate::config::field_config::{
    FieldManualOperation, FieldOperation, FieldOperationType,
};
use crate::utils::parser::ExprOrFunc;

pub struct ValidatorAttr {
    pub items: Punctuated<ValidatorItem, Token![,]>,
}

impl Parse for ValidatorAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: input.parse_terminated(ValidatorItem::parse, Token![,])?,
        })
    }
}

impl ValidatorAttr {
    pub fn into_operations(self) -> Vec<FieldOperation> {
        self.items
            .into_iter()
            .map(|t| t.into_operation().into())
            .collect()
    }
}

pub enum ValidatorItem {
    Plain(PlainValidator),
    Message(MessageValidator),
    Fallible(FallibleValidator),
}

impl Parse for ValidatorItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![try]) {
            Ok(Self::Fallible(input.parse()?))
        } else if input.peek(token::Paren) {
            Ok(Self::Message(input.parse()?))
        } else {
            Ok(Self::Plain(input.parse()?))
        }
    }
}

impl ValidatorItem {
    pub fn into_operation(self) -> FieldManualOperation {
        match self {
            Self::Plain(PlainValidator { expr }) => FieldManualOperation {
                ty: FieldOperationType::Validate,
                expr: expr.expr,
                message: None,
                fallible: false,
            },
            Self::Message(MessageValidator { expr, message, .. }) => {
                FieldManualOperation {
                    ty: FieldOperationType::Validate,
                    expr: expr.expr,
                    message: Some(message),
                    fallible: false,
                }
            }
            Self::Fallible(FallibleValidator { expr, message, .. }) => {
                FieldManualOperation {
                    ty: FieldOperationType::Validate,
                    expr: expr.expr,
                    message,
                    fallible: true,
                }
            }
        }
    }
}

pub struct ValidatorExpr {
    pub expr: ExprOrFunc,
}

impl Parse for ValidatorExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
        })
    }
}

pub struct PlainValidator {
    pub expr: ValidatorExpr,
}

impl Parse for PlainValidator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
        })
    }
}

pub struct MessageValidator {
    pub _paren: token::Paren,
    pub expr: ValidatorExpr,
    pub message: Expr,
}

impl Parse for MessageValidator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _paren: parenthesized!(content in input),
            expr: content.parse()?,
            message: {
                content.parse::<Token![,]>()?;
                content.parse()?
            },
        })
    }
}

pub struct FallibleValidator {
    pub _prefix: Token![try],
    pub _paren: token::Paren,
    pub expr: ValidatorExpr,
    pub message: Option<Expr>,
}

impl Parse for FallibleValidator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _prefix: input.parse()?,
            _paren: parenthesized!(content in input),
            expr: content.parse()?,
            message: {
                if content.peek(Token![,]) && content.peek2(LitStr) {
                    content.parse::<Token![,]>()?;
                    Some(content.parse()?)
                } else {
                    None
                }
            },
        })
    }
}
