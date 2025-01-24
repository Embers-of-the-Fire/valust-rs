use serde::{Deserialize, Serialize};
use valust::{Raw, Validate};
use valust_derive::Valust;

#[test]
fn test_forward_attr() {
    #[derive(Debug, Clone, Valust)]
    #[forward_derive(Debug, Clone, Serialize, Deserialize)]
    #[forward_attr(serde(rename_all = "camelCase"))]
    struct Fwd {
        #[forward_attr(serde(default))]
        #[valid(expr(name_self.len() < 5))]
        name_self: String,
    }

    let text = r#"{ "nameSelf": "abc" }"#;
    let json: Raw<Fwd> = serde_json::from_str(text).unwrap();
    let valid = Fwd::validate(json).unwrap();
    assert_eq!(valid.name_self, "abc");
    println!("{:#?}", valid);
}
