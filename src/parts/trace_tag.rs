//! Provider of [`TraceTag`].

use crate::util::*;

/// Trace tag.
#[derive(Clone, Debug, Default, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct TraceTag {
    /// Addres key.
    key: AddrKey,
    /// `true` if this is for cloned related value.
    for_clone: bool,
}

impl TraceTag {
    /// Creates a new instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tag = TraceTag::new();
    /// assert_eq!(tag.for_clone(), false);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new instance for cloned related value.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tag = TraceTag::new_for_clone();
    /// assert_eq!(tag.for_clone(), true);
    /// ```
    #[must_use]
    pub fn new_for_clone() -> Self {
        Self {
            key: AddrKey::new(),
            for_clone: true,
        }
    }

    /// Returns `true` if this is for cloned related value.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tag1 = TraceTag::new();
    /// let tag2 = TraceTag::new_for_clone();
    /// assert_eq!(tag1.for_clone(), false);
    /// assert_eq!(tag2.for_clone(), true);
    /// ```
    #[must_use]
    pub fn for_clone(&self) -> bool {
        self.for_clone
    }
}
