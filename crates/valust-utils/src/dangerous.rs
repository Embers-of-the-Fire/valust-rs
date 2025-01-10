//! **Dangerous** functions that might cause a panic.

#![allow(private_bounds, private_interfaces)]

use std::fmt::Debug;

trait Unwrap {
    type Out;

    fn unwrap(self) -> Self::Out;

    fn expect(self, msg: &str) -> Self::Out;
}

impl<T> Unwrap for Option<T> {
    type Out = T;

    fn unwrap(self) -> Self::Out {
        Option::unwrap(self)
    }

    fn expect(self, msg: &str) -> Self::Out {
        Option::expect(self, msg)
    }
}

impl<T, E: Debug> Unwrap for Result<T, E> {
    type Out = T;

    fn unwrap(self) -> Self::Out {
        Result::unwrap(self)
    }

    fn expect(self, msg: &str) -> Self::Out {
        Result::expect(self, msg)
    }
}

/// Unwrap an `Option` or `Result`.
/// 
/// This might cause a panic, use carefully.
/// 
/// ```rust,ignore
/// struct MustBeSome(
///     #[trans(Option<i32> => fn(unwrap))]
///     i32,
/// )
/// ```
pub fn unwrap<T: Unwrap>(t: T) -> T::Out {
    t.unwrap()
}

/// Unwrap an `Option` or `Result` with a custom message.
/// 
/// This might cause a panic, use carefully.
/// 
/// ```rust,ignore
/// struct MustBeSome(
///     #[trans(Option<i32> => fn(expect("must be some")))]
///     i32,
/// )
/// ```
pub fn expect<T: Unwrap>(msg: &str) -> impl Fn(T) -> T::Out + '_ {
    move |t| t.expect(msg)
}
