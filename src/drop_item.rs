//! Provider of [`DropItem`].

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Items to monitor drops.
#[derive(Default, Debug)]
pub struct DropItem {
    /// The number of items not dropped.
    count: Arc<AtomicUsize>,
}

impl DropItem {
    /// Create new instance.
    pub(super) fn new(count: &Arc<AtomicUsize>) -> Self {
        Self {
            count: Arc::clone(count),
        }
    }
}

impl Drop for DropItem {
    fn drop(&mut self) {
        self.count.fetch_sub(1, Ordering::SeqCst);
    }
}
