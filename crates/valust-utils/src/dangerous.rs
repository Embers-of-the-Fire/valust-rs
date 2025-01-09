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

pub fn unwrap<T: Unwrap>(t: T) -> T::Out {
    t.unwrap()
}

pub fn expect<T: Unwrap>(msg: &str) -> impl Fn(T) -> T::Out + '_ {
    move |t| t.expect(msg)
}
