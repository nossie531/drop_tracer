//! Provider of [`StatusMap`].

use crate::prelude::*;
use std::collections::BTreeMap;

/// Status map.
pub trait StatusMap {
    /// Returns `true` if given tag is living, or `false` if dropped.
    #[must_use]
    fn get(&self, tag: &TraceTag) -> Option<bool>;

    /// Returns map iterator.
    fn iter(&self) -> impl Iterator<Item = (&TraceTag, bool)>;
}

impl StatusMap for BTreeMap<TraceTag, bool> {
    fn get(&self, tag: &TraceTag) -> Option<bool> {
        self.get(tag).copied()
    }

    fn iter(&self) -> impl Iterator<Item = (&TraceTag, bool)> {
        self.iter().map(|(k, v)| (k, *v))
    }
}
