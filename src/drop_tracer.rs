//! Provider of [`DropTracer`].

use super::drop_item::DropItem;
use super::LeakError;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Memory leak detector.
///
/// A simple memory leak detector that manages the difference
/// between generated and dropped items.
#[derive(Default)]
pub struct DropTracer {
    /// The number of items not dropped.
    count: Arc<AtomicUsize>,
}

impl DropTracer {
    /// Create new instance.
    pub fn new() -> Self {
        Self {
            count: Default::default(),
        }
    }

    /// Tests specified functions for memory leaks.
    pub fn test_drop<F>(f: F)
    where
        F: FnOnce(&mut DropTracer),
    {
        Self::try_drop(f).unwrap();
    }

    /// Checks specified functions for memory leaks.
    pub fn try_drop<F>(f: F) -> Result<(), LeakError>
    where
        F: FnOnce(&mut DropTracer),
    {
        let mut tracer = DropTracer::new();
        f(&mut tracer);
        let count = tracer.count();

        if count == 0 {
            Ok(())
        } else {
            Err(LeakError::new(count))
        }
    }

    /// Returns the number of items not dropped.
    pub fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    /// Generates items for drop monitoring.
    pub fn new_item(&self) -> DropItem {
        self.count.fetch_add(1, Ordering::SeqCst);
        DropItem::new(&self.count)
    }
}
