//! **Dangerous** functions that might cause a panic.

#![allow(private_bounds, private_interfaces)]

use std::fmt::Debug;

use sealed::sealed;

/// This trait is implemented for `Option` and `Result` types.
///
/// This trait is **sealed**.
#[sealed]
pub trait Unwrap {
    /// Out type of the `unwrap` operation.
    ///
    /// This will be `T` for `Option<T>` and `Result<T, E>`.
    type Out;

    #[allow(missing_docs)]
    fn unwrap(self) -> Self::Out;

    #[allow(missing_docs)]
    fn expect(self, msg: &str) -> Self::Out;
}

#[sealed]
impl<T> Unwrap for Option<T> {
    type Out = T;

    fn unwrap(self) -> Self::Out {
        Option::unwrap(self)
    }

    fn expect(self, msg: &str) -> Self::Out {
        Option::expect(self, msg)
    }
}

#[sealed]
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
/// ```rust
/// # use valust_utils::dangerous::unwrap;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct MustBeSome(
///     #[trans(func(Option<i32> => unwrap))]
///     i32,
/// );
/// ```
pub fn unwrap<T: Unwrap>(t: T) -> T::Out {
    t.unwrap()
}

/// Unwrap an `Option` or `Result` with a custom message.
///
/// This might cause a panic, use carefully.
///
/// ```rust
/// # use valust_utils::dangerous::expect;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Valust)]
/// struct MustBeSome(
///     #[trans(func(Option<i32> => expect("must be some")))]
///     i32,
/// );
/// ```
pub fn expect<T: Unwrap>(msg: &str) -> impl Fn(T) -> T::Out + '_ {
    move |t| t.expect(msg)
}
