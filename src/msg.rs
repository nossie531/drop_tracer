//! Crate's messages.

/// Message that detect working tracer.
pub const TRACER_BORROWED: &str = "Tracer borrowed (ex: reading logs).";

/// Message that detect double drop.
pub const DOUBLE_DROPPED: &str = "Double drop is detected.";

/// Message that detect memory leak.
pub fn leak_error(count: usize) -> String {
    format!("Memory leak is dtected on {} values.", count)
}
