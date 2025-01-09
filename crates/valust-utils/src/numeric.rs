pub fn larger_than<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    move |b| a > b
}

pub fn smaller_than<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    move |b| a < b
}

pub fn equal_to<T: PartialEq>(a: T) -> impl Fn(T) -> bool {
    move |b| a == b
}

pub fn not_equal_to<T: PartialEq>(a: T) -> impl Fn(T) -> bool {
    move |b| a != b
}

pub fn larger_than_or_equal_to<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    move |b| a >= b
}

pub fn smaller_than_or_equal_to<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    move |b| a <= b
}

pub fn in_range<T: PartialOrd>(a: T, b: T) -> impl Fn(T) -> bool {
    move |c| a <= c && c <= b
}

pub fn gt<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    larger_than(a)
}

pub fn lt<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    smaller_than(a)
}

pub fn eq<T: PartialEq>(a: T) -> impl Fn(T) -> bool {
    equal_to(a)
}

pub fn ne<T: PartialEq>(a: T) -> impl Fn(T) -> bool {
    not_equal_to(a)
}

pub fn ge<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    larger_than_or_equal_to(a)
}

pub fn le<T: PartialOrd>(a: T) -> impl Fn(T) -> bool {
    smaller_than_or_equal_to(a)
}

pub fn between<T: PartialOrd>(a: T, b: T) -> impl Fn(T) -> bool {
    in_range(a, b)
}
