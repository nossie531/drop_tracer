//! Provider of [`TraceError`].

use crate::*;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// The error type for memory leak.
#[derive(Debug)]
pub(crate) enum TraceError {
    TracerBorrowed,
    DoubleDrop,
}

impl TraceError {
    /// Panics by this error.
    pub fn panic(&self) {
        panic!("{}", format!("{self}"));
    }
}

impl Error for TraceError {}

impl Display for TraceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TraceError::TracerBorrowed => msg::TRACER_BORROWED,
                TraceError::DoubleDrop => msg::DOUBLE_DROPPED,
            }
        )
    }
}
