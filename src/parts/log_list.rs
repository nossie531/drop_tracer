//! Provider of [`LogList`].

use crate::prelude::*;
use std::ops::Index;

/// Log list.
pub trait LogList
where
    Self: Index<usize, Output = TraceLog>,
{
    /// Returns an iterator.
    fn iter(&self) -> impl Iterator<Item = &TraceLog>;

    /// Returns the number of logs.
    fn len(&self) -> usize;

    /// Returns `true` if the log is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl LogList for Vec<TraceLog> {
    fn iter(&self) -> impl Iterator<Item = &TraceLog> {
        <[TraceLog]>::iter(self)
    }

    fn len(&self) -> usize {
        self.len()
    }
}
