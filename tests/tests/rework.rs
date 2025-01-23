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
        #[valid(color(compat, ty = "hsl", prefix = "no"))] String,
        #[forward] S,
    );

    let raw = RawW(
        "1234".into(),
        "hello@world.email".into(),
        "1,2%,3%".into(),
        RawS(10),
    );
    let _valid = W::validate(raw).unwrap();
    // let err = W::validate(raw).unwrap_err();
    // println!("{}", err.full_stringify());
}

#[test]
fn test_regex() {
    let regex = "^(\\d{1,3},\\d{1,3}%,\\d{1,3}%)$";
    let re = valust::regex::Regex::new(regex).unwrap();
    println!("{:?}", re.is_match("1,2%,3%"));
}
