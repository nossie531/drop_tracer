//! Crate's errors.

pub use leak_error::*;

pub(crate) use trace_error::*;

mod leak_error;
mod trace_error;
