# More valid utils

## `valid(expr)`

**Syntax:**
- **Infallible:** `expr(<expr>, <msg>?)`
- **Fallible:** `expr(try(<expr>), <msg>?)`

**Description:**
This will directly insert the expression into the validate function.
Note that all variables is passed by-value, so if you do not want to
consume them, explicitly using a reference.

**Example:**
- Basic: `#[valid(expr(a > 10))]`
- With message: ``#[valid(expr(a > 10, "invalid `a`"))]``
- Fallible: `#[valid(expr(try(<some fallible expr>)))]`

## `valid(func)`

**Syntax:**
- **Infallible:** `func(<func-name>, <msg>?)`
- **Fallible:** `func(try(<func-name>), <msg>?)`

**Description:**
You can also pass function-like expressions like closures.
The expanded code will be like `(<func>)(&<field>)`.
The function must accept a single value by-ref.

**Example:**
- Basic: `#[valid(func(|a| a > 10))]`
- Fallible `#[valid(func( |s| s.parse::<u8>().map(|t| t > 10) ))]`

## `valid(regex)`

**Required feature:** `regex`

**Syntax:**
`regex(<regex-expr>, <msg>?)`

**Description:**
The regex backend is [`regex::Regex`][regex].

**`Sync` & `Send`:**
The regex state machine is wrapped in a [`std::sync::LazyLock`][lazy-lock]
and will be initialized as a `static` item inside the impl block.

[regex]: https://docs.rs/regex/latest/regex/struct.Regex.html
[lazy-lock]: https://doc.rust-lang.org/std/sync/struct.LazyLock.html

**Example:**
- Basic: `#[valid(regex("\d{4}-\d{2}-\d{2}"))]`
- With message: `#[valid(regex("\d{4}-\d{2}-\d{2}", "invalid date"))]`

## `valid(email)`

**Required feature:** `regex`, `regex-utils`, `email`

**Syntax:** `email`

**Description:**
Checks if a string-like value is a valid email address.

**Example:** `#[valid(email)]`

**Reference:** [`valust-regex-utils > EMAIL`][email]

[email]: https://docs.rs/valust-regex-utils/latest/valust_regex_utils/constant.EMAIL.html

## `valid(url)`

**Required feature:** `regex`, `regex-utils`, `url`

**Syntax:** `url`

**Description:**
Checks if a string-like value is a valid URL.

**Example:** `#[valid(url)]`

**Reference:** [`valust-regex-utils > URL`][url]

[url]: https://docs.rs/valust-regex-utils/latest/valust_regex_utils/constant.URL.html

## `valid(username)`

**Required feature:** `regex`, `regex-utils`, `username`

**Syntax:** `username`

**Description:**
Checks if a string-like value is a valid username.

**Example:** `#[valid(username)]`

**Reference:** [`valust-regex-utils > USERNAME`][username]

[username]: https://docs.rs/valust-regex-utils/latest/valust_regex_utils/constant.USERNAME.html

