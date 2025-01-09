# Valust-Derive

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
use the [`rename` struct attribute](#structure-attributes).

## Syntax

### Field Attributes

| Name      | Syntax                            | Description                                                                                                                                                 | Example                                  |
| --------- | --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| `valid`   | `valid(<validator>)`              | [Validator](#validator-expression), uses _raw_ data as input.                                                                                               | `valid(a > 10)`                          |
| `trans`   | `trans(<transformer>)`            | [Transformer](#transformer-expression), converts _raw_ data to _transformed_ data.                                                                          | `trans(try(String => s.parse::<u32>()))` |
| `forward` | `forward`, `forward(Ident)`       | [Forward the field](#forwarding-a-field) and uses it's validator to validate the field. itself.                                                             | `forward(RawInput)`                      |
| `display` | `display = bool`, `display(bool)` | Whether to display the value when an error occurs. Enabled by default. <br /> [**Performance issues.**](#performance-issues-when-displaying-error-messages) | `display = false`                        |

### Structure Attributes

| Name             | Syntax                              | Description                                           | Example                     |
| ---------------- | ----------------------------------- | ----------------------------------------------------- | --------------------------- |
| `forward_derive` | `forward_derive(Ident)`             | Add derive macros for the _raw_ data struct.          | `forward_derive(Debug)`     |
| `pre`            | `pre(<validator>)`                  | Pre-validator, which uses _raw_ data as input.        | `pre(a > 10, b + c != 0.0)` |
| `post`           | `post(<validator>)`                 | Post-validator, which uses _validated_ data as input. | `post(a > 10)`              |
| `rename`         | `rename = "Ident"`, `rename(Ident)` | Rename the output _raw_ data struct.                  | `rename(Original)`          |

### Special Expressions

#### Validator Expression

- Plain validator:
  - Syntax: `valid(<expr>)`
  - Example: `valid(a > 10)`
- Validator with custom message:
  - Syntax: `valid((<expr>, <msg>))`
  - Example: ``valid((b != 0, "`b` must be non-zero"))``
- Fallible validator:
  - Syntax: `valid(try(<expr>))`
  - Example: `valid(try(s.parse::<u32>() > 10))`
- Fallible validator with message:
  - Syntax: `valid(try(<expr>, <msg>))`
  - Example: ``valid(try(s.parse::<u32>() > 10, "`s` must be non-zero-string))``

#### Transformer Expression

- Plain transformer:
  - Syntax: `trans(Type => <expr>)`
  - Example: `trans(String => s.trim())`
- Fallible transformer:
  - Syntax: `trans(try(Type => <expr>))`
  - Example: `trans(try(String => s.parse::<u32>()))`
- Fallible transformer with message:
  - Syntax: `trans(try(Type => <expr>, <msg>))`
  - Example: `trans(try(String => s.parse::<u32>(), "fail to parse number"))`

## Example

```rust
use valust::Validate;
use valust_utils::convert::parse_to;
    
#[derive(Debug, Valust)]
#[forward_derive(Debug)]
pub struct Inner {
    #[display = true]
    #[valid(code > 10.0)]
    pub code: f64,
}

#[derive(Debug, Valust)]
#[forward_derive(Debug)]
pub struct Outer {
    // #[forward(InnerPre)]
    #[forward]
    pub inner: Inner,
    #[trans(String => extra.trim())]
    #[trans(try(String => fn(parse_to::<u32>)))]
    pub extra: u32,
}
```

### Macro Expansion

<details>

```rust
#[derive(Debug)]
pub struct RawInner {
    pub code: f64,
}
impl ::valust::Validate<Inner> for RawInner {
    fn validate(
        self,
    ) -> ::std::result::Result<Inner, ::valust::error::ValidationError> {
        #![allow(non_snake_case)]
        let RawInner { code } = self;
        let mut __valust_error_Inner = ::valust::error::ValidationError::new();
        fn _valust_process_code(
            code: f64,
            _valust_error: &mut ::valust::error::ValidationError,
        ) -> ::std::option::Option<f64> {
            if !(code > 10.0) {
                _valust_error.push_validate_error(::valust::error::ValidateError {
                    field: "code",
                    path: format!("{}", "code"),
                    value: format!("(f64) {:?}", code),
                    cause: ::std::boxed::Box::new(::valust::error::ValidateFail),
                    message: ::std::option::Option::None,
                    expression: "code > 10.0",
                    type_name: "f64",
                });
            }
            ::std::option::Option::Some(code)
        }
        let code: ::std::option::Option<f64> =
            _valust_process_code(code, &mut __valust_error_Inner);
        __valust_error_Inner.check()?;
        let mut __valust_error_Inner = ::valust::error::ValidationError::new();
        let code =
            code.expect("Unexpected error occurred while processing field `code`");
        __valust_error_Inner.check()?;
        ::std::result::Result::Ok(Inner { code })
    }
}

#[derive(Debug)]
pub struct RawOuter {
    pub inner: RawInner,
    pub extra: String,
}
impl ::valust::Validate<Outer> for RawOuter {
    fn validate(
        self,
    ) -> ::std::result::Result<Outer, ::valust::error::ValidationError> {
        #![allow(non_snake_case)]
        let RawOuter { inner, extra } = self;
        let mut __valust_error_Outer = ::valust::error::ValidationError::new();
        fn _valust_process_inner(
            inner: RawInner,
            _valust_error: &mut ::valust::error::ValidationError,
        ) -> ::std::option::Option<Inner> {
            let inner = match (<RawInner as ::valust::Validate<Inner>>::validate(inner))
            {
                Ok(value) => value,
                Err(__valust_err_inner) => {
                    _valust_error.extend_error("inner", __valust_err_inner);
                    return None;
                }
            };
            ::std::option::Option::Some(inner)
        }
        let inner: ::std::option::Option<Inner> =
            _valust_process_inner(inner, &mut __valust_error_Outer);
        fn _valust_process_extra(
            extra: String,
            _valust_error: &mut ::valust::error::ValidationError,
        ) -> ::std::option::Option<u32> {
            let extra = extra.trim();
            let __format_err_clone_extra = extra.clone();
            let extra = match (parse_to::<u32>(extra)) {
                Ok(value) => value,
                Err(__valust_err_extra) => {
                    _valust_error.push_transform_error(
                        ::valust::error::TransformError {
                            field: "extra",
                            path: format!("{}", "extra"),
                            value: format!("(String) {:?}", __format_err_clone_extra),
                            cause: ::std::boxed::Box::new(__valust_err_extra),
                            message: ::std::option::Option::None,
                            expression: "parse_to :: < u32 > (extra)",
                            source_type_name: "String",
                            target_type_name: "u32",
                        },
                    );
                    return None;
                }
            };
            ::std::option::Option::Some(extra)
        }
        let extra: ::std::option::Option<u32> =
            _valust_process_extra(extra, &mut __valust_error_Outer);
        __valust_error_Outer.check()?;
        let mut __valust_error_Outer = ::valust::error::ValidationError::new();
        let inner =
            inner.expect("Unexpected error occurred while processing field `inner`");
        let extra =
            extra.expect("Unexpected error occurred while processing field `extra`");
        __valust_error_Outer.check()?;
        ::std::result::Result::Ok(Outer { inner, extra })
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

If no parameter is provided to `forward`, it will automatically use the default
naming pattern (i.e., `RawXXX`).

### Performance issues when displaying error messages

Displaying **huge** data may lead to performance issues, as the internal
formatter will `clone` the data for fear that user-defined expressions might
take the field by-value instead of by-ref.
