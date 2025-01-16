use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitStr, Token, parenthesized, token};

use crate::config::field_config::{
    FieldManualOperation, FieldOperation, FieldOperationType,
};
use crate::utils::parser::Expression;

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
    #[cfg(feature = "regex")]
    Regex(RegexValidator),
}

impl Parse for ValidatorItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if fork.peek(Token![try]) {
            return Ok(Self::Fallible(input.parse()?));
        } else if fork.peek(Ident) {
            let ident: Ident = fork.parse()?;
            #[cfg(feature = "regex")]
            if ident == "regex" {
                input.parse::<Ident>()?;
                return Ok(Self::Regex(input.parse()?));
            }
            #[cfg(not(feature = "regex"))]
            if ident == "regex" {
                return Err(syn::Error::new(
                    ident.span(),
                    crate::utils::err_text::REGEX_NOT_ENABLED,
                ));
            }
        } else if input.peek(token::Paren) {
            return Ok(Self::Message(input.parse()?));
        }

        Ok(Self::Plain(input.parse()?))
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
            #[cfg(feature = "regex")]
            Self::Regex(RegexValidator { text, message, .. }) => FieldManualOperation {
                ty: FieldOperationType::Validate,
                expr: text,
                message,
                fallible: false,
            },
        }
    }
}

pub struct ValidatorExpr {
    pub expr: Expression,
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

#[cfg(feature = "regex")]
pub struct RegexValidator {
    pub _paren: token::Paren,
    pub text: Expression,
    pub message: Option<Expr>,
}

#[cfg(feature = "regex")]
impl Parse for RegexValidator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _paren: parenthesized!(content in input),
            text: Expression::Regex(content.parse::<LitStr>()?),
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
