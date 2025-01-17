//! Time-related regex expressions.

/// Regex for matching a valid 12-hour time format (HH:MM) with no `am/pm` suffix.
///
/// Note this regex allows omitting the leading zeros.
///
/// ## Example
///
/// ```rust
/// use regex::Regex;
/// use valust_regex_utils::time::HH_MM_12H_NO_LEADING_NO_SUFFIX;
///
/// let time_regex = Regex::new(HH_MM_12H_NO_LEADING_NO_SUFFIX).unwrap();
/// assert!(time_regex.is_match("1:00"));
/// assert!(time_regex.is_match("12:00"));
/// assert!(!time_regex.is_match("13:00"));
///
/// assert!(time_regex.is_match("1:59"));
/// assert!(!time_regex.is_match("1:60"));
/// ```
pub const HH_MM_12H_NO_LEADING_NO_SUFFIX: &str = r"^(0?[1-9]|1[0-2]):[0-5][0-9]$";

/// Regex for matching a valid 12-hour time format (HH:MM) with `am/pm` suffix.
///
/// Note this regex allows omitting the leading zeros.
///
/// ## Example
///
/// ```rust
/// use regex::Regex;
/// use valust_regex_utils::time::HH_MM_12H_NO_LEADING;
///
/// let time_regex = Regex::new(HH_MM_12H_NO_LEADING).unwrap();
///
/// assert!(time_regex.is_match("1:00 am"));
/// assert!(time_regex.is_match("12:00 pm"));
/// assert!(!time_regex.is_match("13:00 am"));
///
/// assert!(time_regex.is_match("1:59 pm"));
/// assert!(!time_regex.is_match("1:60 am"));
/// ```
pub const HH_MM_12H_NO_LEADING: &str =
    r"^((1[0-2]|0?[1-9]):([0-5][0-9]) ?([AaPp][Mm]))$";

/// Regex for matching a valid 24-hour time format (HH:MM).
///
/// Note this regex does not allow omitting the leading zeros.
///
/// ## Example
///
/// ```rust
/// use regex::Regex;
/// use valust_regex_utils::time::HH_MM_24H;
///
/// let time_regex = Regex::new(HH_MM_24H).unwrap();
///
/// assert!(time_regex.is_match("00:00"));
/// assert!(time_regex.is_match("23:59"));
/// assert!(!time_regex.is_match("24:00"));
///
/// assert!(time_regex.is_match("01:59"));
/// assert!(!time_regex.is_match("1:60"));
/// ```
pub const HH_MM_24H: &str = r"^(0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$";

/// Regex for matching a valid 24-hour time format (HH:MM).
///
/// Note this regex allows omitting the leading zeros.
///
/// ## Example
///
/// ```rust
/// use regex::Regex;
/// use valust_regex_utils::time::HH_MM_24H_NO_LEADING;
///
/// let time_regex = Regex::new(HH_MM_24H_NO_LEADING).unwrap();
///
/// assert!(time_regex.is_match("0:00"));
/// assert!(time_regex.is_match("23:59"));
/// assert!(!time_regex.is_match("24:00"));
///
/// assert!(time_regex.is_match("1:59"));
/// assert!(!time_regex.is_match("1:60"));
/// ```
pub const HH_MM_24H_NO_LEADING: &str = r"^([0-9]|0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$";

/// Regex for matching a valid 24-hour time format (HH:MM:SS).
///
/// Note this regex does not allow omitting the leading zeros.
///
/// ## Example
///
/// ```rust
/// use regex::Regex;
/// use valust_regex_utils::time::HH_MM_SS;
///
/// let time_regex = Regex::new(HH_MM_SS).unwrap();
///
/// assert!(time_regex.is_match("00:00:00"));
/// assert!(time_regex.is_match("23:59:59"));
/// assert!(!time_regex.is_match("24:00:00"));
///
/// assert!(time_regex.is_match("01:59:59"));
/// assert!(!time_regex.is_match("1:60:00"));
/// ```
pub const HH_MM_SS: &str = r"^(?:[01]\d|2[0123]):(?:[012345]\d):(?:[012345]\d)$";
