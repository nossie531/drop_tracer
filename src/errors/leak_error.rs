//! Provider of [`LeakError`].

use crate::*;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// The error type for memory leak.
#[derive(Debug)]
pub struct LeakError {
    /// The number of leaked values.
    count: usize,
}

impl LeakError {
    /// Creates a new instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::errors::*;
    ///
    /// let result = LeakError::new(42);
    /// assert_eq!(result.count(), 42);
    /// ```
    #[must_use]
    pub fn new(count: usize) -> Self {
        LeakError { count }
    }

    /// Returns detected count of memory leaks.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let result = drop_test::try_run(|tracer| {
    ///     std::mem::forget(tracer.trace("val1"));
    ///     std::mem::forget(tracer.trace("val2"));
    /// });
    ///
    /// assert_eq!(result.err().unwrap().count(), 2);
    #[must_use]
    pub fn count(&self) -> usize {
        self.count
    }
}

impl Error for LeakError {}

impl Display for LeakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", msg::leak_error(self.count))
    }
}
