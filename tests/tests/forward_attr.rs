use valust::{Validate, Raw};
use valust_derive::Valust;
use serde::{Serialize, Deserialize};

#[test]
fn test_forward_attr() {
    #[derive(Debug, Clone, Valust)]
    #[forward_derive(Debug, Clone, Serialize, Deserialize)]
    struct Fwd {
        #[forward_attr(serde(default))]
        #[valid(expr(name.len() < 5))]
        name: String,
    }

    let text = r#"{ "name": "abc" }"#;
    let json: Raw<Fwd> = serde_json::from_str(text).unwrap();
    let valid = Fwd::validate(json).unwrap();
    println!("{:#?}", valid);
}