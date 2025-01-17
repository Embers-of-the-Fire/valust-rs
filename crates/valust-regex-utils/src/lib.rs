#![doc = include_str!("../README.md")]

pub mod time;

/// Regex for matching a valid email address.
///
/// ## Example
///
/// ```rust
/// use regex::Regex;
/// use valust_regex_utils::EMAIL;
///
/// let email_regex = Regex::new(EMAIL).unwrap();
/// assert!(email_regex.is_match("hello@world.com"));
/// ```
///
/// ## Reference
///
/// - [HTML Standard / Email Address](https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address)
pub const EMAIL: &str = concat!(
    r"^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+",
    r"@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
);

/// Regex for matching a syntax-valid URL.
///
/// This regex does not check if the URL is reachable, e.g., if the IP address is valid.
///
/// ## Example
///
/// ```rust
/// use regex::Regex;
/// use valust_regex_utils::URL;
///
/// let url_regex = Regex::new(URL).unwrap();
/// assert!(url_regex.is_match("https://www.google.com"));
/// ```
pub const URL: &str = r"^(https?:\/\/)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)$";

/// Regex for matching a valid ascii username with a minimum length of 3 characters.
///
/// ## Example
///
/// ```rust
/// use regex::Regex;
/// use valust_regex_utils::USERNAME;
///
/// let username_regex = Regex::new(USERNAME).unwrap();
/// assert!(username_regex.is_match("hello-world"));
/// assert!(!username_regex.is_match("hi"));
/// ```
pub const USERNAME: &str = r"^[a-zA-Z0-9_-]{3,}$";

/// Regex for matching a valid hex color (`#aabbcc`/`#abc`).
///
/// ## Example
///
/// ```rust
/// use regex::Regex;
/// use valust_regex_utils::HEX_COLOR;
///
/// let color_regex = Regex::new(HEX_COLOR).unwrap();
/// assert!(color_regex.is_match("#00ff00"));
/// assert!(color_regex.is_match("#123"));
/// assert!(color_regex.is_match("#223344ff"));
/// ```
pub const HEX_COLOR: &str =
    r"^#?([a-fA-F0-9]{8}|[a-fA-F0-9]{6}|[a-fA-F0-9]{4}|[a-fA-F0-9]{3})$";
