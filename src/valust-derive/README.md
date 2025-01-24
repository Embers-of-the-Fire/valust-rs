# Valust-Derive

<center>

<img alt="Crate Version" src="https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fgithub.com%2FEmbers-of-the-Fire%2Fvalust-rs%2Fraw%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.workspace.package.version&prefix=v%20&style=for-the-badge&label=version">&emsp;
<img alt="GitHub top language" src="https://img.shields.io/github/languages/top/embers-of-the-fire/valust-rs?style=for-the-badge&color=%23FF9B07">&emsp;
<a href="https://crates.io/crates/valust-derive">
    <img alt="Crates.io Downloads (recent)" src="https://img.shields.io/crates/dr/valust-derive?style=for-the-badge">
</a>

</center>

---

## Output

The derive macro `Valust` will emit both an extra structure containing _raw_
data and an implementation of `valust::Validator`.

The _raw_ data `struct` is identical to the structure definition given to the
`Valust` macro (i.e., `struct A { a: A }` becomes `struct RawA { a: A }`, and
`struct B(B)` becomes `struct RawB(B)`). The fields are automatically derived by
`Valust`. Specifically, fields with only `valid` retain their type, fields with
one or more `trans` have the type of the first `transformer` in the first
`trans`, and fields with `forward` are determined by the
[`forward` field](#forwarding-a-field).

The default naming pattern of the _raw_ data struct is `RawXXX`. To override it,
use the [`rename` struct attribute](#rename).

## Syntax

### Generic Attributes

These attributes can be used on either fields or the structure itself.

#### `forward_attr`

|             |                                                  |
| ----------- | ------------------------------------------------ |
| Syntax      | `forward_attr(<attribute>)`                      |
| Description | Add external attribute to the _raw_ data type    |
| Example     | `#[forward_attr(serde(rename_all="camelCase"))]` |

### Field Attributes

#### `valid`

|             |                                   |
| ----------- | --------------------------------- |
| Syntax      | `valid(<valid exprs>)`            |
| Description | Add a validator to check a field. |
| Example     | `#[valid(expr(a > 10))]`          |

**Reference:** [valid expr](#validator-expression)

#### `trans`

|             |                                         |
| ----------- | --------------------------------------- |
| Syntax      | `trans(<trans exprs>)`                  |
| Description | Add a transformer to modify a field.    |
| Example     | `#[trans(expr(try(a.parse::<u32>())))]` |

**Reference:** [trans expr](#transformer-expression)

#### `forward`

|             |                                           |
| ----------- | ----------------------------------------- |
| Syntax      | `forward`                                 |
| Description | [Forward](#forwarding-a-field) the field. |
| Example     | `#[forward]`                              |

### Structure Attributes

#### `rename`

|             |                                                            |
| ----------- | ---------------------------------------------------------- |
| Syntax      | `rename(<ident>)` or `rename = "<ident>"`(not recommended) |
| Description | [Rename](#why-we-need-rename) the _raw_ data structure.    |
| Example     | `#[rename(RawData)]` or `#[rename = "RawData"]`            |

#### `forward_derive`

|             |                                                     |
| ----------- | --------------------------------------------------- |
| Syntax      | `forward_derive(<derive-items>)`                    |
| Description | Add `derive` attribute to the _raw_ data structure. |
| Example     | `#[forward_derive(Debug, Clone)]`                   |

#### `pre`

|             |                                                           |
| ----------- | --------------------------------------------------------- |
| Syntax      | `pre(<struct-valid-expr>)`                                |
| Description | Add struct-level validator before all field validators.   |
| Example     | `#[pre((magic1 + magic2 == 10, "invalid magic number"))]` |

**Reference:** [struct-valid-expr](#struct-level-validator-expression)

#### `post`

|             |                                                            |
| ----------- | ---------------------------------------------------------- |
| Syntax      | `post(<struct-valid-expr>)`                                |
| Description | Add struct-level validator after all field validators.     |
| Example     | `#[post((magic1 + magic2 == 10, "invalid magic number"))]` |

**Reference:** [struct-valid-expr](#struct-level-validator-expression)

### Special Expressions

#### Struct-level Validator Expression

- Plain validator:
  - Syntax: `<expr>`
  - Example: `a > 10`
  - Full attr: `#[pre(a > 10)]`
- Validator with custom message:
  - Syntax: `(<expr>, <msg>)`
  - Example: ``(b != 0, "`b` must be non-zero")``
  - Full attr: ``#[pre(b != 0, "`b` must be non-zero")]``

#### Validator Expression

[**Reference**](./valid-utils.md)

#### Transformer Expression

[**Reference**](./trans-utils.md)

## Example

```rust
use valust::Validate;
use valust_derive::Valust;
use valust_utils::convert::parse_to;
    
#[derive(Debug, Valust)]
#[forward_derive(Debug)]
pub struct Inner {
    #[valid(expr(code > 10.0, "code must be greater than 10.0"))]
    pub code: f64,
}

#[derive(Debug, Valust)]
#[forward_derive(Debug)]
pub struct Outer {
    #[forward]
    pub inner: Inner,
    #[trans(expr(String => extra.trim()))]
    #[trans(func(String => try(parse_to::<u32>)))]
    pub extra: u32,
}
```

### Macro Expansion

<details>

```rust,ignore
#[automatically_derived]
#[derive(Debug)]
pub struct RawInner {
    pub code: f64,
}
#[automatically_derived]
#[allow(
    non_camel_case_types,
    non_snake_case,
    unused_variables,
    non_upper_case_globals
)]
impl ::valust::Validate for Inner {
    type Raw = RawInner;
    fn validate(raw: Self::Raw) -> Result<Self, ::valust::error::ValidationError> {
        let RawInner { code } = raw;
        let mut valust_impl_err_Inner = ::valust::error::ValidationError::new();
        valust_impl_err_Inner.check()?;
        let mut valust_impl_err_Inner = ::valust::error::ValidationError::new();
        fn valust_validate_code(
            code: f64,
            valust_err_code: &mut ::valust::error::ValidationError,
        ) -> Option<f64> {
            if !({ code > 10.0 }) {
                valust_err_code.push_validate_error(
                    ::valust::error::validate::ValidateError {
                        field: "code",
                        path: format!("{}", "code"),
                        value: format!("(f64) {:?}", code),
                        cause: ::std::option::Option::None,
                        message: ::std::option::Option::Some(
                            "code must be greater than 10.0",
                        ),
                        expression: "{code > 10.0}",
                        type_name: "f64",
                    },
                );
                return None;
            }
            Some(code)
        }
        let code: Option<f64> = valust_validate_code(code, &mut valust_impl_err_Inner);
        valust_impl_err_Inner.check()?;
        let code = code.expect("Unexpected error occurred in processing field `code`");
        let mut valust_impl_err_Inner = ::valust::error::ValidationError::new();
        valust_impl_err_Inner.check()?;
        Ok(Inner { code })
    }
}

#[automatically_derived]
#[derive(Debug)]
pub struct RawOuter {
    pub inner: ::valust::Raw<Inner>,
    pub extra: String,
}
#[automatically_derived]
#[allow(
    non_camel_case_types,
    non_snake_case,
    unused_variables,
    non_upper_case_globals
)]
impl ::valust::Validate for Outer {
    type Raw = RawOuter;
    fn validate(raw: Self::Raw) -> Result<Self, ::valust::error::ValidationError> {
        let RawOuter { inner, extra } = raw;
        let mut valust_impl_err_Outer = ::valust::error::ValidationError::new();
        valust_impl_err_Outer.check()?;
        let mut valust_impl_err_Outer = ::valust::error::ValidationError::new();
        fn valust_validate_inner(
            inner: ::valust::Raw<Inner>,
            valust_err_inner: &mut ::valust::error::ValidationError,
        ) -> Option<Inner> {
            let inner: Inner = match ::valust::Validate::validate(inner) {
                Ok(v_valust) => v_valust,
                Err(e_valust) => {
                    valust_err_inner.extend_error("inner", e_valust);
                    return None;
                }
            };
            Some(inner)
        }
        let inner: Option<Inner> =
            valust_validate_inner(inner, &mut valust_impl_err_Outer);
        fn valust_validate_extra(
            extra: String,
            valust_err_extra: &mut ::valust::error::ValidationError,
        ) -> Option<u32> {
            let extra = ({ extra.trim() });
            let extra = {
                let valust_format_err_clone_extra = extra.clone();
                match ((parse_to::<u32>)(extra)) {
                    ::std::result::Result::Ok(valust_v) => valust_v,
                    ::std::result::Result::Err(valust_trans_err_cause) => {
                        valust_err_extra.push_transform_error(
                            ::valust::error::transform::TransformError {
                                field: "extra",
                                path: format!("{}", "extra"),
                                value: format!(
                                    "(String) {:?}",
                                    valust_format_err_clone_extra
                                ),
                                cause: ::std::boxed::Box::new(valust_trans_err_cause),
                                message: ::std::option::Option::Some(
                                    "`extra`'s transform expression fails",
                                ),
                                expression: "(parse_to :: < u32 >) (extra)",
                                source_type_name: "String",
                                target_type_name: "<unknown>",
                            },
                        );
                        return None;
                    }
                }
            };
            Some(extra)
        }
        let extra: Option<u32> =
            valust_validate_extra(extra, &mut valust_impl_err_Outer);
        valust_impl_err_Outer.check()?;
        let inner =
            inner.expect("Unexpected error occurred in processing field `inner`");
        let extra =
            extra.expect("Unexpected error occurred in processing field `extra`");
        let mut valust_impl_err_Outer = ::valust::error::ValidationError::new();
        valust_impl_err_Outer.check()?;
        Ok(Outer { inner, extra })
    }
}
```

</details>

## Appendix

### `Forward`ing a field

Fields that implement `valust::Validator` are not automatically recognized by
Valust, but Valust can leverage pre-defined Validator implementations.
Specifically, by using the `forward` field attribute, `Valust` will execute the
`valust::Validator::validate` method of the field's type, automatically
extending the error `path` field. Additionally, it will automatically change the
corresponding field of the _raw_ data struct to the original data type of that
field.

The _raw_ data type could be inferred by the compiler, so you don't need to
specify it even if you've `rename`d it.

### Regex validator

`valust-derive` supports regex-based validator expressions using [`regex`](https://crates.io/crates/regex).
To enable regex support, you must enable the `regex` feature for both `valust` and `valust-derive`.

**Note:**
`valust > regex` feature will enable `valust-derive > regex` if `derive` feature is enabled.

### Why we need `rename`

Though we don't need to specify the _raw_ data type when executing the
validator, we might need to construct them directly. Sadly, due to rust's
language syntax limitations, we cannot construct an unnamed struct using type
alias (i.e. `valust::Raw::<Foo>`). That's why we may need `rename` a _raw_
type.

### Performance issues when displaying error messages

Displaying **huge** data may lead to performance issues, as the internal
formatter will `clone` the data for fear that user-defined expressions might
take the field by-value instead of by-ref.
