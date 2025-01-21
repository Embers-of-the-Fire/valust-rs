#![allow(unused_comparisons, clippy::absurd_extreme_comparisons, dead_code)]

use valust::Raw;
use valust::error::display::ErrorDisplay;
use valust_derive::Valust;

#[test]
fn test_macro() {
    use valust::Validate;

    #[derive(Debug, Valust)]
    #[post(_0 + *_1 as f64 > 10.0)]
    pub struct W(
        #[trans(expr(f64 => _0.abs()))]
        #[valid(expr(_0 > 10.0, "Failed check"))]
        pub f64,
        #[trans(expr(String => try(_1.parse::<u32>())))]
        #[valid(expr(_1 < 0))]
        pub u32,
    );

    println!(
        "{:#?}",
        W::validate(RawW(-2.0, "15".to_owned())).map(|_| "")
    );
}

#[test]
fn test_nested() {
    use valust::Validate;
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

    let out = Outer::validate(Raw::<Outer> {
        inner: Raw::<Inner> { code: 10.0 },
        extra: "  1a".to_owned(),
    });
    println!("{:#?}\n", out);
    let err = out.unwrap_err();
    println!("{}", err.full_stringify());
    println!("{}", err.brief_stringify());
    println!("{}", err.human_readable_stringify());
}
