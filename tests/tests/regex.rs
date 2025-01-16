use valust::error::display::ErrorDisplay;
use valust::{Raw, Validate};
use valust_derive::Valust;

#[test]
fn test_regex() {
    #[derive(Debug, Valust)]
    #[forward_derive(Debug)]
    struct Test {
        #[valid(regex(r"^\d{4}-\d{2}-\d{2}$", "Invalid date format"))]
        #[allow(unused)]
        date: String,
    }

    let raw = Raw::<Test> {
        date: "2021-01-0".to_string(),
    };
    let valid = Test::validate(raw);
    println!("{}", valid.unwrap_err().full_stringify());
}
