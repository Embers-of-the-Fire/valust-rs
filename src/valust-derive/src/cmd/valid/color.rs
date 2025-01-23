use proc_macro2::{Span, TokenStream};
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{Expr, Ident, LitStr, Token};

use super::regex::gen_regex_expr;
use super::{ValidCommand, ValidHandler};
use crate::utils::require_bool::require_bool_expr;
use crate::utils::require_lit_str::require_lit_str;
use crate::utils::require_single::require_single_fallible;

pub struct ColorCommand;

impl ValidCommand for ColorCommand {
    fn ident(&self) -> &'static str {
        "color"
    }

    fn parse_inner(&self, tt: ParseStream) -> syn::Result<Box<dyn ValidHandler>> {
        #[cfg(not(feature = "color"))]
        return Err(syn::Error::new(tt.span(), "feature `color` is not enabled"));

        #[cfg(feature = "color")]
        // color = "<rgb | hsl | ... >"
        if tt.peek(Token![=]) {
            tt.parse::<Token![=]>()?;
            let lit: LitStr = tt.parse()?;
            let span = lit.span();
            let ty = ColorType::from_text(lit)?;
            Ok(Box::new(ColorHandler {
                span,
                color_type: ty,
                prefix: Default::default(),
                compat: false,
            }))
        }
        // color(xxx)
        // type, prefix, compat
        else if tt.peek(Paren) {
            let base_span = tt.span();
            let mut color_type: Option<ColorType> = None;
            let mut prefix = None;
            let mut compat = None;
            let content;
            syn::parenthesized!(content in tt);

            fn parse_kv(buf: ParseStream) -> syn::Result<(Ident, Option<Expr>)> {
                let ident: Ident = buf.parse()?;
                if buf.peek(Token![=]) {
                    buf.parse::<Token![=]>()?;
                    let expr: Expr = buf.parse()?;
                    return Ok((ident, Some(expr)));
                }
                Ok((ident, None))
            }

            for (ident, expr) in
                Punctuated::<_, Token![,]>::parse_terminated_with(&content, parse_kv)?
            {
                match ident.to_string().as_str() {
                    "ty" => require_single_fallible(
                        expr.map(|e| require_lit_str(e).and_then(ColorType::from_text)),
                        &mut color_type,
                        "type",
                        ident.span(),
                    )?,
                    "prefix" => require_single_fallible(
                        expr.map(|e| {
                            require_lit_str(e).and_then(PrefixPermission::from_text)
                        }),
                        &mut prefix,
                        "prefix",
                        ident.span(),
                    )?,
                    "compat" => {
                        if let Some(expr) = expr {
                            require_single_fallible(
                                Some(require_bool_expr(expr)),
                                &mut compat,
                                "compat",
                                ident.span(),
                            )?
                        } else {
                            compat = Some(true);
                        }
                    }
                    _ => {}
                }
            }

            Ok(Box::new(ColorHandler {
                span: base_span,
                color_type: color_type.unwrap_or_default(),
                prefix: prefix.unwrap_or_default(),
                compat: compat.unwrap_or(true),
            }))
        } else {
            Ok(Box::new(ColorHandler {
                span: tt.span(),
                color_type: Default::default(),
                prefix: Default::default(),
                compat: true,
            }))
        }
    }
}

struct ColorHandler {
    span: Span,
    color_type: ColorType,
    prefix: PrefixPermission,
    compat: bool,
}

impl ColorHandler {
    pub fn to_regex(&self) -> String {
        self.color_type.to_regex(self.compat, self.prefix)
    }
}

impl ValidHandler for ColorHandler {
    fn gen_expr_display(&self, _field: &Ident) -> Option<String> {
        Some(self.to_regex())
    }

    fn message(&self, field: &Ident) -> Option<String> {
        Some(format!("`{}` is not a valid color literal", field))
    }

    fn is_fallible(&self) -> bool {
        false
    }

    fn gen_validator_expr(&self, field: &Ident) -> TokenStream {
        gen_regex_expr(field, self.to_regex(), self.span)
    }
}

#[derive(Default, Clone, Copy)]
enum ColorType {
    Rgb,
    Hsl,
    #[default]
    Hex,
}

impl ColorType {
    pub fn from_text(ty: LitStr) -> syn::Result<Self> {
        const COLOR_ERROR_TEXT: &str = "unknown color type";

        match ty.value().to_ascii_lowercase().as_str() {
            "rgb" => Ok(Self::Rgb),
            "hsl" => Ok(Self::Hsl),
            "hex" => Ok(Self::Hex),
            _ => Err(syn::Error::new(ty.span(), COLOR_ERROR_TEXT)),
        }
    }

    pub fn to_regex(self, compat: bool, perm: PrefixPermission) -> String {
        use ColorType::{Hex, Hsl, Rgb};

        let (code, is_hex) = match (self, compat) {
            (Rgb, true) => (r"\d{1,3},\d{1,3}\d{1,3}", false),
            (Rgb, false) => (r"\s*\d{1,3}\s*,\s*\d{1,3}\s*,\s*\d{1,3}\s*", false),
            (Hsl, true) => (r"\d{1,3},\d{1,3}%,\d{1,3}%", false),
            (Hsl, false) => {
                (r"\s*\d{1,3}\s*,\s*\d{1,3}\s*%\s*,\s*\d{1,3}\s*%\s*", false)
            }
            (Hex, true) => (r"([0-9a-fA-F]{2}){3}|([0-9a-fA-F]){3}", true),
            (Hex, false) => (r"((\s*[0-9a-fA-F]{2}){3}|(\s*[0-9a-fA-F]){3})\s*", true),
        };

        if is_hex {
            perm.format_regex_hex(code)
        } else {
            perm.format_regex(code, self.to_type_text())
        }
    }

    pub fn to_type_text(self) -> &'static str {
        use ColorType::{Hex, Hsl, Rgb};

        match self {
            Rgb => "rgb",
            Hsl => "hsl",
            Hex => "hex",
        }
    }
}

#[derive(Default, Clone, Copy)]
enum PrefixPermission {
    No,
    Force,
    #[default]
    Accept,
}

impl PrefixPermission {
    pub fn from_text(ty: LitStr) -> syn::Result<Self> {
        const PREFIX_PERM_ERROR_TEXT: &str = "unknown prefix permission";

        match ty.value().to_ascii_lowercase().as_str() {
            "no" => Ok(Self::No),
            "force" => Ok(Self::Force),
            "accept" => Ok(Self::Force),
            _ => Err(syn::Error::new(ty.span(), PREFIX_PERM_ERROR_TEXT)),
        }
    }

    pub fn format_regex(self, core: &str, ty: &str) -> String {
        match self {
            Self::No => format!("^({core})$"),
            Self::Force => format!(r"^({ty}\({core}\))$"),
            Self::Accept => format!(r"^((({ty})?\({core}\))|({core}))$"),
        }
    }

    pub fn format_regex_hex(self, hex: &str) -> String {
        match self {
            Self::No => format!("^({})$", hex),
            Self::Force => format!("^(0x{})$", hex),
            Self::Accept => format!("^((0x)?{})$", hex),
        }
    }
}
