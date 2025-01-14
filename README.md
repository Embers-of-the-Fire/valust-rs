# Valust - Validator for Rust

`Valust` aims to provide a user-friendly, auto-generated data validation tool.

By leveraging Rust's powerful procedural macro system, `valust` can
automatically generate everything you need for data validation with just a few
simple attributes added to your data structures.

## Example

```rust
use valust::Validate;
use valust_derive::Valust;

#[derive(Debug, Valust, PartialEq)]
#[forward_derive(Debug)]
#[rename(UncheckedUsername)]
pub struct Username(
    #[trans(String => _0.trim().to_owned())]
    #[valid((!_0.is_empty(), "username must not be empty"))]
    pub String,
);

#[derive(Debug, Valust, PartialEq)]
#[forward_derive(Debug)]
#[post(user_id + (username.0.len() as u32) == magic_number)]
pub struct UserProfile {
    pub user_id: u32,
    #[forward]
    pub username: Username,
    pub magic_number: u32,
}

let raw_profile = Raw::<UserProfile> {
    user_id: 10,
    username: UncheckedUsername("  Foo  ".into()),
    magic_number: 13,
};

let profile = UserProfile::validate(raw_profile).expect("Check failed");

assert_eq!(profile, UserProfile {
    user_id: 10,
    username: Username("Foo".into()),
    magic_number: 13
});
```

## Project Structure

### Core

- [`valust`](https://crates.io/crates/valust): Main crate, exposing fundamental traits (`trait Validate`) and error types.
- [`valust-derive`](https://crates.io/crates/valust-derive): Derive macro for creating a validate-able struct.

### External Tools & Utilities

- [`valust-utils`](https://crates.io/crates/valust-utils): Utilities that might be used when defining validators.

## Minimum Supported Rust Version (MSRV)

The MSRV of this project is 1.78.0 (With lockfile version 4),
but is possibly able to compile unlocked with Rust 1.74.1 (With lockfile version 3).

And for development, the project requires Rust 1.83.0 to work on the source code.
