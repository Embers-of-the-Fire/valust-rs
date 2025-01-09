#![allow(unused_comparisons, clippy::absurd_extreme_comparisons, dead_code)]

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

    println!("{:#?}", RawW(-2.0, "15".to_owned()).validate().map(|_| ""));
}

#[test]
fn test_nested() {
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

    println!(
        "{:#?}",
        RawOuter {
            inner: RawInner { code: 10.0 },
            extra: "  1".to_owned(),
        }
        .validate()
    );
}
