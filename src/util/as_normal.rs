//! Provider of [`AsNormal`].

use std::ops::Deref;
use std::panic::AssertUnwindSafe;

/// A wrapper that asserts normal behavior.
///
/// Overrides the characteristics [`UnwindSafe`] and [`RefUnwindSafe`] of the wrapped type.
///
/// [`UnwindSafe`]: std::panic::UnwindSafe
/// [`RefUnwindSafe`]: std::panic::RefUnwindSafe
#[derive(Debug)]
pub(crate) struct AsNormal<T>(AssertUnwindSafe<T>);

impl<T> AsNormal<T> {
    /// Creates a new instance.
    pub fn new(value: T) -> Self {
        Self(AssertUnwindSafe(value))
    }
}

impl<T> Clone for AsNormal<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(AssertUnwindSafe(self.0.clone()))
    }
}

impl<T> Default for AsNormal<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(AssertUnwindSafe(Default::default()))
    }
}

impl<T> Deref for AsNormal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
