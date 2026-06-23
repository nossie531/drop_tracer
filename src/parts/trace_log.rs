//! Provider of [`TraceLog`].

use crate::prelude::*;

/// Trace log.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceLog {
    /// Trace tag.
    tag: TraceTag,
    /// `true` if allocated, or `false` if dropped.
    alloc: bool,
}

impl TraceLog {
    /// Creates a new instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tag = &TraceTag::new();
    /// let log = &TraceLog::new(tag, true);
    /// assert_eq!(log.tag(), tag);
    /// assert_eq!(log.is_alloc(), true);
    /// ```
    #[must_use]
    pub fn new(tag: &TraceTag, alloc: bool) -> Self {
        let tag = tag.clone();
        Self { tag, alloc }
    }

    /// Creates a new instance by trace value.
    ///
    /// This is shorthand of [`new`](Self::new) and [`TraceVal::tag`].
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let val = &TraceVal::new(42);
    /// let log = &TraceLog::new_by_val(val, true);
    /// assert_eq!(log.tag(), &TraceVal::tag(val));
    /// assert_eq!(log.is_alloc(), true);
    /// ```
    #[must_use]
    pub fn new_by_val<T>(val: &TraceVal<T>, alloc: bool) -> Self {
        Self::new(&TraceVal::tag(val), alloc)
    }

    /// Returns a reference to trace tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tag = &TraceTag::new();
    /// let log = &TraceLog::new(tag, true);
    /// assert_eq!(log.tag(), tag);
    /// ```
    #[must_use]
    pub fn tag(&self) -> &TraceTag {
        &self.tag
    }

    /// Returns `true` if allocated, or `false` if dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tag = &TraceTag::new();
    /// let log = &TraceLog::new(tag, true);
    /// assert_eq!(log.is_alloc(), true);
    /// ```
    #[must_use]
    pub fn is_alloc(&self) -> bool {
        self.alloc
    }

    /// Returns `true` if this log is about cloned value.
    ///
    /// This is shorthand of [`tag`] and [`TraceTag::for_clone`].
    /// 
    /// [`tag`]: Self::tag
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tag1 = &TraceTag::new();
    /// let tag2 = &TraceTag::new_for_clone();
    /// let log1 = &TraceLog::new(tag1, true);
    /// let log2 = &TraceLog::new(tag2, true);
    /// assert_eq!(log1.is_cloned(), false);
    /// assert_eq!(log2.is_cloned(), true);
    /// ```
    #[must_use]
    pub fn is_cloned(&self) -> bool {
        self.tag.for_clone()
    }
}
