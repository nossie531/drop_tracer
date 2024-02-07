//! Provider of [`LeakError`].

use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// The error type for memory leak.
#[derive(Debug)]
pub struct LeakError {
    count: usize,
}

impl LeakError {
    /// Create a new value.
    pub fn new(count: usize) -> Self {
        LeakError { count }
    }

    /// Returns detected count of memory leaks.
    pub fn count(&self) -> usize {
        self.count
    }
}

impl Error for LeakError {}

impl Display for LeakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Memory leak is dtected on {} items.", self.count)
    }
}
