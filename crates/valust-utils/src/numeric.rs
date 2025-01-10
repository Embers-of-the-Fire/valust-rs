//! Utilities for numeric validations.

/// Returns a closure that takes an argument and checks if it is less than the given value `a`.
/// 
/// This can be passed to `#[valid]` attribute like: `#[valid(fn(larger_than(5))]`.
pub fn larger_than<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    move |b| a > b
}

/// Returns a closure that takes an argument and checks if it is greater than the given value `a`.
/// 
/// This can be passed to `#[valid]` attribute like: `#[valid(fn(smaller_than(5))]`.
pub fn smaller_than<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    move |b| a < b
}

/// Returns a closure that takes an argument and checks if it is equal to the given value `a`.
/// 
/// This can be passed to `#[valid]` attribute like: `#[valid(fn(equal_to(5))]`.
pub fn equal_to<T: PartialEq>(a: T) -> impl Fn(T) -> bool {
    move |b| a == b
}

/// Returns a closure that takes an argument and checks if it is not equal to the given value `a`.
/// 
/// This can be passed to `#[valid]` attribute like: `#[valid(fn(not_equal_to(5))]`.
pub fn not_equal_to<T: PartialEq>(a: T) -> impl Fn(T) -> bool {
    move |b| a != b
}

/// Returns a closure that takes an argument and checks if it is greater than or equal to the given value `a`.
/// 
/// This can be passed to `#[valid]` attribute like: `#[valid(fn(larger_than_or_equal_to(5))]`.
pub fn larger_than_or_equal_to<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    move |b| a >= b
}

/// Returns a closure that takes an argument and checks if it is less than or equal to the given value `a`.
/// 
/// This can be passed to `#[valid]` attribute like: `#[valid(fn(smaller_than_or_equal_to(5))]`.
pub fn smaller_than_or_equal_to<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    move |b| a <= b
}

/// Returns a closure that takes an argument and checks if it is within the given range `[a, b]`.
/// 
/// This can be passed to `#[valid]` attribute like: `#[valid(fn(in_range(1, 5))]`.
pub fn in_range<T: PartialOrd>(a: T, b: T) -> impl Fn(T) -> bool {
    move |c| a <= c && c <= b
}

/// Alias for [`larger_than`].
pub fn gt<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    larger_than(a)
}

/// Alias for [`smaller_than`].
pub fn lt<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    smaller_than(a)
}

/// Alias for [`equal_to`].
pub fn eq<T: PartialEq>(a: T) -> impl Fn(T) -> bool {
    equal_to(a)
}

/// Alias for [`not_equal_to`].
pub fn ne<T: PartialEq>(a: T) -> impl Fn(T) -> bool {
    not_equal_to(a)
}

/// Alias for [`larger_than_or_equal_to`].
pub fn ge<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    larger_than_or_equal_to(a)
}

/// Alias for [`smaller_than_or_equal_to`].
pub fn le<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    smaller_than_or_equal_to(a)
}

/// Alias for [`in_range`].
pub fn between<T: PartialOrd>(a: T, b: T) -> impl Fn(T) -> bool {
    in_range(a, b)
}
