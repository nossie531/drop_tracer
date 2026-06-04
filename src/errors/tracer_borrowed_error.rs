//! Provider of [`TracerBorrowedError`].

use crate::*;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// The error type for memory leak.
#[derive(Debug)]
pub(crate) struct TracerBorrowedError();

impl TracerBorrowedError {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Panics by this error.
    pub fn panic(&self) {
        panic!("{}", format!("{self}"));
    }
}

impl Error for TracerBorrowedError {}

impl Display for TracerBorrowedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", msg::TRACER_BORROWED)
    }
}
