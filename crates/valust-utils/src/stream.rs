//! Utilities for working with streams, like `vec`.
//!
//! ## Functions overview
//!
//! - `vec`:
//!     - validator: [`all_vec`], [`try_vec`]
//!     - transformer: [`map_vec`], [`try_map_vec`]
//! - `HashSet`:
//!     - validator: [`all_set`], [`try_set`]
//!     - transformer: Not yet.

use std::collections::{BTreeMap, HashMap, HashSet};

/// Checks if all elements in the [`Vec`] satisfy the predicate.
///
/// ```rust
/// # use valust_utils::stream::all_vec;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(all_vec(|&x| x > 1)))]
///     data: Vec<u8>
/// }
///
/// let all = Raw::<All> { data: vec![1, 2, 3] };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn all_vec<I>(predicate: fn(&I) -> bool) -> impl Fn(&Vec<I>) -> bool {
    move |i| i.iter().all(predicate)
}

/// Checks if all elements in the [`Vec`] satisfy the predicate (fallible ones accepted).
///
/// ```rust
/// # use valust_utils::stream::try_vec;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(try(try_vec(|x: &String| x.parse::<i32>().map(|u| u > 1)))))]
///     data: Vec<String>
/// }
///
/// let all = Raw::<All> { data: vec!["1".to_owned(), "2".to_owned(), "3".to_owned()] };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn try_vec<I, E>(
    predicate: fn(&I) -> Result<bool, E>,
) -> impl Fn(&Vec<I>) -> Result<bool, E> {
    move |i| {
        i.iter()
            .find_map(|i| match predicate(i) {
                Ok(false) => Some(Ok(false)),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .unwrap_or(Ok(true))
    }
}

/// Convert all elements in the [`Vec`] to a new type.
///
/// ```rust
/// # use valust_utils::stream::map_vec;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct Map {
///     #[trans(func(Vec<String> => map_vec(|x: String| x.trim().to_string())))]
///     items: Vec<String>
/// }
///
/// let map = Raw::<Map> { items: vec![" 1 ".to_string(), "2".to_string()] };
/// let val = Map::validate(map);
/// assert_eq!(vec!["1".to_string(), "2".to_string()], val.unwrap().items);
/// ```
pub fn map_vec<I, O>(op: fn(I) -> O) -> impl Fn(Vec<I>) -> Vec<O> {
    move |i| i.into_iter().map(op).collect()
}

/// Checks if all elements in the [`Vec`] satisfy the predicate (fallible ones accepted).
///
/// ```rust
/// # use valust_utils::stream::try_map_vec;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[trans(func(Vec<String> => try(try_map_vec(|x: String| x.parse::<i32>()))))]
///     data: Vec<i32>
/// }
///
/// let all = Raw::<All> { data: vec!["1".to_owned(), "2".to_owned(), "3".to_owned()] };
/// let val = All::validate(all);
/// assert!(val.is_ok());
/// assert_eq!(vec![1, 2, 3], val.unwrap().data);
/// ```
pub fn try_map_vec<I, O, E>(
    predicate: fn(I) -> Result<O, E>,
) -> impl Fn(Vec<I>) -> Result<Vec<O>, E> {
    move |i| i.into_iter().map(predicate).collect()
}

/// Checks if all elements in the [`HashSet`] satisfy the predicate.
///
/// ```rust
/// # use std::collections::HashSet;
/// # use valust_utils::stream::all_set;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(all_set(|&x| x > 1)))]
///     data: HashSet<u8>
/// }
///
/// let all = Raw::<All> { data: vec![1, 2, 3].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn all_set<I>(predicate: fn(&I) -> bool) -> impl Fn(&HashSet<I>) -> bool {
    move |i| i.iter().all(predicate)
}

/// Checks if all elements in the [`HashSet`] satisfy the predicate (fallible ones accepted).
///
/// ```rust
/// # use std::collections::HashSet;
/// # use valust_utils::stream::try_set;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(try(try_set(|x: &String| x.parse::<i32>().map(|u| u > 1)))))]
///     data: HashSet<String>
/// }
///
/// let all = Raw::<All> { data: vec!["1".to_owned(), "2".to_owned(), "3".to_owned()].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn try_set<I, E>(
    predicate: fn(&I) -> Result<bool, E>,
) -> impl Fn(&HashSet<I>) -> Result<bool, E> {
    move |i| {
        i.iter()
            .find_map(|i| match predicate(i) {
                Ok(false) => Some(Ok(false)),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .unwrap_or(Ok(true))
    }
}

/// Checks if all entries in the [`HashMap`] satisfy the predicate.
///
/// ```rust
/// # use std::collections::HashMap;
/// # use valust_utils::stream::all_map;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(all_map(|&k, &v| k > 1 && v > 1)))]
///     data: HashMap<u8, u8>
/// }
///
/// let all = Raw::<All> { data: vec![(1, 2), (2, 3), (3, 4)].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn all_map<K, V>(predicate: fn(&K, &V) -> bool) -> impl Fn(&HashMap<K, V>) -> bool {
    move |i| i.iter().all(|(k, v)| predicate(k, v))
}

/// Checks if all entries in the [`HashMap`] satisfy the predicate (fallible ones accepted).
///
/// ```rust
/// # use std::collections::HashMap;
/// # use valust_utils::stream::try_map;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(try(try_map(|k: &String, v: &String| k.parse::<i32>().map(|u| u > 1 && v.parse::<i32>().unwrap() > 1)))))]
///     data: HashMap<String, String>
/// }
///
/// let all = Raw::<All> { data: vec![("1".to_owned(), "2".to_owned()), ("2".to_owned(), "3".to_owned()), ("3".to_owned(), "4".to_owned())].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn try_map<K, V, E>(
    predicate: fn(&K, &V) -> Result<bool, E>,
) -> impl Fn(&HashMap<K, V>) -> Result<bool, E> {
    move |i| {
        i.iter()
            .find_map(|(k, v)| match predicate(k, v) {
                Ok(false) => Some(Ok(false)),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .unwrap_or(Ok(true))
    }
}

/// Checks if all keys in the [`HashMap`] satisfy the predicate.
///
/// ```rust
/// # use std::collections::HashMap;
/// # use valust_utils::stream::all_map_keys;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(all_map_keys(|&k| k > 1)))]
///     data: HashMap<u8, u8>
/// }
///
/// let all = Raw::<All> { data: vec![(1, 2), (2, 3), (3, 4)].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn all_map_keys<K, V>(
    predicate: fn(&K) -> bool,
) -> impl Fn(&HashMap<K, V>) -> bool {
    move |i| i.keys().all(predicate)
}
/// Checks if all keys in the [`HashMap`] satisfy the predicate (fallible ones accepted).
///
/// ```rust
/// # use std::collections::HashMap;
/// # use valust_utils::stream::try_all_map_keys;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(try(try_all_map_keys(|k: &String| k.parse::<i32>().map(|u| u > 1)))))]
///     data: HashMap<String, u8>
/// }
///
/// let all = Raw::<All> { data: vec![("1".to_owned(), 2), ("2".to_owned(), 3), ("3".to_owned(), 4)].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn try_all_map_keys<K, V, E>(
    predicate: fn(&K) -> Result<bool, E>,
) -> impl Fn(&HashMap<K, V>) -> Result<bool, E> {
    move |i| {
        i.keys()
            .find_map(|k| match predicate(k) {
                Ok(false) => Some(Ok(false)),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .unwrap_or(Ok(true))
    }
}

/// Checks if all values in the [`HashMap`] satisfy the predicate.
///
/// ```rust
/// # use std::collections::HashMap;
/// # use valust_utils::stream::all_map_values;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(all_map_values(|&v| v > 3)))]
///     data: HashMap<u8, u8>
/// }
///
/// let all = Raw::<All> { data: vec![(1, 2), (2, 3), (3, 4)].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn all_map_values<K, V>(
    predicate: fn(&V) -> bool,
) -> impl Fn(&HashMap<K, V>) -> bool {
    move |i| i.values().all(predicate)
}

/// Checks if all values in the [`HashMap`] satisfy the predicate (fallible ones accepted).
///
/// ```rust
/// # use std::collections::HashMap;
/// # use valust_utils::stream::try_all_map_values;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(try(try_all_map_values(|v: &String| v.parse::<i32>().map(|u| u > 3)))))]
///     data: HashMap<u8, String>
/// }
///
/// let all = Raw::<All> { data: vec![(1, "2".to_owned()), (2, "3".to_owned()), (3, "4".to_owned())].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn try_all_map_values<K, V, E>(
    predicate: fn(&V) -> Result<bool, E>,
) -> impl Fn(&HashMap<K, V>) -> Result<bool, E> {
    move |i| {
        i.values()
            .find_map(|v| match predicate(v) {
                Ok(false) => Some(Ok(false)),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .unwrap_or(Ok(true))
    }
}

/// Checks if all entries in the [`BTreeMap`] satisfy the predicate.
///
/// ```rust
/// # use std::collections::BTreeMap;
/// # use valust_utils::stream::all_btree_map;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(all_btree_map(|&k, &v| k > 1 && v > 1)))]
///     data: BTreeMap<u8, u8>
/// }
///
/// let all = Raw::<All> { data: vec![(1, 2), (2, 3), (3, 4)].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn all_btree_map<K, V>(
    predicate: fn(&K, &V) -> bool,
) -> impl Fn(&BTreeMap<K, V>) -> bool {
    move |i| i.iter().all(|(k, v)| predicate(k, v))
}

/// Checks if all entries in the [`BTreeMap`] satisfy the predicate (fallible ones accepted).
///
/// ```rust
/// # use std::collections::BTreeMap;
/// # use valust_utils::stream::try_btree_map;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(try(try_btree_map(|k: &String, v: &String| k.parse::<i32>().map(|u| u > 1 && v.parse::<i32>().unwrap() > 1)))))]
///     data: BTreeMap<String, String>
/// }
///
/// let all = Raw::<All> { data: vec![("1".to_owned(), "2".to_owned()), ("2".to_owned(), "3".to_owned()), ("3".to_owned(), "4".to_owned())].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn try_btree_map<K, V, E>(
    predicate: fn(&K, &V) -> Result<bool, E>,
) -> impl Fn(&BTreeMap<K, V>) -> Result<bool, E> {
    move |i| {
        i.iter()
            .find_map(|(k, v)| match predicate(k, v) {
                Ok(false) => Some(Ok(false)),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .unwrap_or(Ok(true))
    }
}

/// Checks if all keys in the [`BTreeMap`] satisfy the predicate.
///
/// ```rust
/// # use std::collections::BTreeMap;
/// # use valust_utils::stream::all_btree_map_keys;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(all_btree_map_keys(|&k| k > 1)))]
///     data: BTreeMap<u8, u8>
/// }
///
/// let all = Raw::<All> { data: vec![(1, 2), (2, 3), (3, 4)].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn all_btree_map_keys<K, V>(
    predicate: fn(&K) -> bool,
) -> impl Fn(&BTreeMap<K, V>) -> bool {
    move |i| i.keys().all(predicate)
}

/// Checks if all values in the [`BTreeMap`] satisfy the predicate.
///
/// ```rust
/// # use std::collections::BTreeMap;
/// # use valust_utils::stream::all_btree_map_values;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(all_btree_map_values(|&v| v > 3)))]
///     data: BTreeMap<u8, u8>
/// }
///
/// let all = Raw::<All> { data: vec![(1, 2), (2, 3), (3, 4)].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn all_btree_map_values<K, V>(
    predicate: fn(&V) -> bool,
) -> impl Fn(&BTreeMap<K, V>) -> bool {
    move |i| i.values().all(predicate)
}

/// Checks if all keys in the [`BTreeMap`] satisfy the predicate (fallible ones accepted).
///
/// ```rust
/// # use std::collections::BTreeMap;
/// # use valust_utils::stream::try_btree_map_keys;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(try(try_btree_map_keys(|k: &String| k.parse::<i32>().map(|u| u > 1)))))]
///     data: BTreeMap<String, u8>
/// }
///
/// let all = Raw::<All> { data: vec![("1".to_owned(), 2), ("2".to_owned(), 3), ("3".to_owned(), 4)].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn try_btree_map_keys<K, V, E>(
    predicate: fn(&K) -> Result<bool, E>,
) -> impl Fn(&BTreeMap<K, V>) -> Result<bool, E> {
    move |i| {
        i.keys()
            .find_map(|k| match predicate(k) {
                Ok(false) => Some(Ok(false)),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .unwrap_or(Ok(true))
    }
}

/// Checks if all values in the [`BTreeMap`] satisfy the predicate (fallible ones accepted).
///
/// ```rust
/// # use std::collections::BTreeMap;
/// # use valust_utils::stream::try_btree_map_values;
/// # use valust::{Validate, Raw, error::display::ErrorDisplay};
/// # use valust_derive::Valust;
/// #
/// #[derive(Debug, Valust)]
/// struct All {
///     #[valid(func(try(try_btree_map_values(|v: &String| v.parse::<i32>().map(|u| u > 3)))))]
///     data: BTreeMap<u8, String>
/// }
///
/// let all = Raw::<All> { data: vec![(1, "2".to_owned()), (2, "3".to_owned()), (3, "4".to_owned())].into_iter().collect() };
/// let val = All::validate(all);
/// assert!(val.is_err());
/// println!("{}", val.unwrap_err().full_stringify());
/// ```
pub fn try_btree_map_values<K, V, E>(
    predicate: fn(&V) -> Result<bool, E>,
) -> impl Fn(&BTreeMap<K, V>) -> Result<bool, E> {
    move |i| {
        i.values()
            .find_map(|v| match predicate(v) {
                Ok(false) => Some(Ok(false)),
                Err(e) => Some(Err(e)),
                _ => None,
            })
            .unwrap_or(Ok(true))
    }
}
