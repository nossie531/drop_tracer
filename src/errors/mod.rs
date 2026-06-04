//! Crate's errors.

pub use leak_error::*;

pub(crate) use tracer_borrowed_error::*;

mod leak_error;
mod tracer_borrowed_error;
