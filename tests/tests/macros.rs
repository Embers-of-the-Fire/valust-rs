#![allow(unused_comparisons, clippy::absurd_extreme_comparisons, dead_code)]

use valust::Raw;
use valust::error::display::ErrorDisplay;
use valust_derive::Valust;

#[test]
fn test_macro() {
    use valust::Validate;

    #[derive(Debug, Valust)]
    #[post(_0 + _1 as f64 > 10.0)]
    pub struct W(
        #[trans((f64 => _0.abs()))]
        #[valid((_0 > 10.0, "Failed check"))]
        pub f64,
        #[trans(try(String => _1.parse::<u32>()))]
        #[valid(_1 < 0)]
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
        #[display = true]
        #[valid((code > 10.0, "code must be greater than 10.0"))]
        pub code: f64,
    }

    #[derive(Debug, Valust)]
    #[forward_derive(Debug)]
    pub struct Outer {
        #[forward]
        pub inner: Inner,
        #[trans(String => extra.trim())]
        #[trans(try(String => fn(parse_to::<u32>)))]
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
