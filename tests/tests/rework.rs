use valust::error::display::ErrorDisplay;

#[test]
fn test_rework() {
    use valust::Validate;
    use valust_derive::Valust;

    #[derive(Debug, Valust)]
    struct S(u32);

    #[derive(Debug, Valust)]
    struct W(
        // #[forward(Forward)]
        #[valid(regex(r"^\d{4}$"), expr(_0.as_str() == "1234", "abc"))] String,
        #[valid(email)] String,
        #[forward] S,
    );

    let raw = RawW("1234".into(), "helloworld.email".into(), RawS(10));
    // let _valid = W::validate(raw).unwrap();
    let err = W::validate(raw).unwrap_err();
    println!("{}", err.full_stringify());
}
