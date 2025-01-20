#[test]
fn real_world_test() -> Result<(), Box<dyn std::error::Error>> {
    use valust::{Raw, Validate};
    use valust_derive::Valust;

    #[derive(Debug, Valust, PartialEq)]
    #[forward_derive(Debug)]
    #[rename(UncheckedUsername)]
    pub struct Username(
        #[trans(expr(String => _0.trim().to_owned()))]
        #[valid(expr(!_0.is_empty(), "username must not be empty"))]
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

    Ok(())
}
