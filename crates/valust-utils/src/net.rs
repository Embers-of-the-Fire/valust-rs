//! Network related validators.

use std::net::{
    AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6,
};

/// Check if a string-like item is a valid IP address.
///
/// ```rust
/// # use valust_utils::net::is_ip;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct Stringify {
///     #[valid(func(is_ip))]
///     ip: String,
/// }
/// ```
pub fn is_ip(s: impl AsRef<str>) -> bool {
    s.as_ref().parse::<IpAddr>().is_ok()
}

/// Convert a string-like item into an IP address.
///
/// ```rust
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// use valust_utils::net::ip as trans_ip;
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(func(String => try(trans_ip)))]
///     ip: std::net::IpAddr,
/// }
/// ```
pub fn ip(s: impl AsRef<str>) -> Result<IpAddr, AddrParseError> {
    s.as_ref().parse()
}

/// Check if a string-like item is a valid IPv4 address.
///
/// ```rust
/// # use valust_utils::net::is_ipv4;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct Stringify {
///     #[valid(func(is_ipv4))]
///     ipv4: String,
/// }
/// ```
pub fn is_ipv4(s: impl AsRef<str>) -> bool {
    s.as_ref().parse::<Ipv4Addr>().is_ok()
}

/// Convert a string-like item into an IPv4 address.
///
/// ```rust
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// use valust_utils::net::ipv4;
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(func(String => try(ipv4)))]
///     ip: std::net::Ipv4Addr,
/// }
/// ```
pub fn ipv4(s: impl AsRef<str>) -> Result<Ipv4Addr, AddrParseError> {
    s.as_ref().parse()
}

/// Check if a string-like item is a valid IPv6 address.
///
/// ```rust
/// # use valust_utils::net::is_ipv6;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct Stringify {
///     #[valid(func(is_ipv6))]
///     ipv6: String,
/// }
/// ```
pub fn is_ipv6(s: impl AsRef<str>) -> bool {
    s.as_ref().parse::<Ipv6Addr>().is_ok()
}

/// Convert a string-like item into an IPv6 address.
///
/// ```rust
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// use valust_utils::net::ipv6;
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(func(String => try(ipv6)))]
///     ip: std::net::Ipv6Addr,
/// }
/// ```
pub fn ipv6(s: impl AsRef<str>) -> Result<Ipv6Addr, AddrParseError> {
    s.as_ref().parse()
}

/// Check if a string-like item is a valid Socket address.
///
/// ```rust
/// # use valust_utils::net::is_sock;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct Stringify {
///     #[valid(func(is_sock))]
///     ip: String,
/// }
/// ```
pub fn is_sock(s: impl AsRef<str>) -> bool {
    s.as_ref().parse::<SocketAddr>().is_ok()
}

/// Convert a string-like item into a Socket address.
///
/// ```rust
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// use valust_utils::net::sock;
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(func(String => try(sock)))]
///     socket: std::net::SocketAddr,
/// }
/// ```
pub fn sock(s: impl AsRef<str>) -> Result<SocketAddr, AddrParseError> {
    s.as_ref().parse()
}

/// Check if a string-like item is a valid SocketV4 address.
///
/// ```rust
/// # use valust_utils::net::is_sockv4;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct Stringify {
///     #[valid(func(is_sockv4))]
///     socket_v4: String,
/// }
/// ```
pub fn is_sockv4(s: impl AsRef<str>) -> bool {
    s.as_ref().parse::<SocketAddrV4>().is_ok()
}

/// Convert a string-like item into a SocketV4 address.
///
/// ```rust
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// use valust_utils::net::sockv4;
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(func(String => try(sockv4)))]
///     socket_v4: std::net::SocketAddrV4,
/// }
/// ```
pub fn sockv4(s: impl AsRef<str>) -> Result<SocketAddrV4, AddrParseError> {
    s.as_ref().parse()
}

/// Check if a string-like item is a valid SocketV6 address.
///
/// ```rust
/// # use valust_utils::net::is_sockv6;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct Stringify {
///     #[valid(func(is_sockv6))]
///     socket_v6: String,
/// }
/// ```
pub fn is_sockv6(s: impl AsRef<str>) -> bool {
    s.as_ref().parse::<SocketAddrV6>().is_ok()
}

/// Convert a string-like item into an IPv6 address.
///
/// ```rust
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// use valust_utils::net::sockv6;
/// #[derive(Valust)]
/// struct Stringify {
///     #[trans(func(String => try(sockv6)))]
///     socket_v6: std::net::SocketAddrV6,
/// }
/// ```
pub fn sockv6(s: impl AsRef<str>) -> Result<SocketAddrV6, AddrParseError> {
    s.as_ref().parse()
}
