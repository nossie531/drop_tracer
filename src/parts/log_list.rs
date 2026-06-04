//! Provider of [`LogList`].

use crate::prelude::*;

/// Log list.
pub trait LogList {
    /// Returns an iterator.
    fn iter(&self) -> impl Iterator<Item = &TraceLog>;
}

impl LogList for Vec<TraceLog> {
    fn iter(&self) -> impl Iterator<Item = &TraceLog> {
        <[TraceLog]>::iter(self)
    }
}
