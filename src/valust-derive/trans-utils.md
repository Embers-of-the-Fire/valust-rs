# More trans utils

## `trans(expr)`

**Syntax:**
`expr(<in-type>? => <expr> => <out-type>?)`
- **Infallible:** `<expr> = <any valid expression>`
- **Fallible:** `<expr> = try(...)`

**Description:**
Both `in-type` and `out-type` could be omitted.
The two type could provide more information for `rustc`'s type inference.
Note that the ever first `in-type` will be the _raw_ field type.

**Example:**
- Basic: `#[trans(expr(a + 1))]`
- Changing type: `#[trans(expr(String => try(s.parse::<u8>())))]`

## `trans(func)`

**Syntax:**
`func(<in-type>? => <func> => <out-type>?)`
- **Infallible:** `<func> = <any valid func-expr>`
- **Fallible:** `<func> = try(...)`

**Description:**
Both `in-type` and `out-type` could be omitted.
The two type could provide more information for `rustc`'s type inference.
Note that the ever first `in-type` will be the _raw_ field type.

**Example:**
- Basic: `#[trans(func(|a| a + 1))]`
- Changing type: `#[trans(func(String => try(|s| s.parse::<u8>())))]`
