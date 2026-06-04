//! Common test runners.

use crate::errors::LeakError;
use crate::prelude::DropTracer;

/// Tests specified functions for memory leaks.
///
/// # Examples
///
/// The following code succeeds.
///
/// ```
/// use drop_tracer::prelude::*;
///
/// drop_test::run(|tracer| {
///     std::mem::drop(tracer.trace("test"));
/// });
/// ```
///
/// The following code fails due to a memory leak.
///
/// ```should_panic
/// use drop_tracer::prelude::*;
///
/// drop_test::run(|tracer| {
///     std::mem::forget(tracer.trace("test"));
/// });
/// ```
pub fn run<F>(f: F)
where
    F: FnOnce(&mut DropTracer),
{
    let result = try_run(f);
    if let Some(err) = result.err() {
        panic!("{}", err);
    }
}

/// Checks specified functions for memory leaks.
///
/// # Examples
///
/// The following code has no memory leak.
///
/// ```
/// use drop_tracer::prelude::*;
///
/// let result = drop_test::try_run(|tracer| {
///     std::mem::drop(tracer.trace("test"));
/// });
///
/// assert!(result.is_ok());
/// ```
///
/// The following code has memory leak.
///
/// ```
/// use drop_tracer::prelude::*;
///
/// let result = drop_test::try_run(|tracer| {
///     std::mem::forget(tracer.trace("test"));
/// });
///
/// assert!(result.is_err());
/// ```
pub fn try_run<F>(f: F) -> Result<(), LeakError>
where
    F: FnOnce(&mut DropTracer),
{
    let mut tracer = DropTracer::new();
    f(&mut tracer);
    let count = tracer.living_count();

    if count == 0 {
        Ok(())
    } else {
        Err(LeakError::new(count))
    }
}
