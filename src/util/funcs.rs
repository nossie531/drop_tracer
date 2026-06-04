//! Utility functions.

use std::cmp::Ordering;
use std::ops::{Add, Sub};

/// Compare two pointers.
pub(crate) fn cmp_ptr<X: ?Sized, Y: ?Sized>(x: *const X, y: *const Y) -> Ordering {
    let x = x.cast::<()>();
    let y = y.cast::<()>();
    x.cmp(&y)
}

/// Returns sum or difference of two values.
pub(crate) fn offset<T>(x: T, y: T, flag: bool) -> T
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
{
    let f = if flag { Add::add } else { Sub::sub };
    f(x, y)
}
