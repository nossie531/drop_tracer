//! Provider of [`DropTracer`].

use crate::parts::*;
use crate::*;
use std::cell::{Ref, RefCell};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Drop tracer.
/// 
/// See the [crate level documentation](crate) for more details.
#[derive(Clone, Debug, Default)]
pub struct DropTracer(Arc<RefCell<DropTracerBody>>);

/// Body of drop tracer.
#[derive(Debug, Default)]
pub(crate) struct DropTracerBody {
    /// The number of living values.
    living_count: usize,
    /// Tracing logs.
    logs: Vec<TraceLog>,
    /// States (living or dropped) map for each values.
    map: BTreeMap<TraceTag, bool>,
}

impl DropTracer {
    /// Creates a new instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tracer = &DropTracer::new();
    /// assert_eq!(tracer.living_count(), 0);
    ///
    /// let item = tracer.trace("some");
    /// assert_eq!(tracer.living_count(), 1);
    ///
    /// drop(item);
    /// assert_eq!(tracer.living_count(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the number of living items.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tracer = &DropTracer::new();
    /// assert_eq!(tracer.living_count(), 0);
    ///
    /// let item = tracer.trace("some");
    /// assert_eq!(tracer.living_count(), 1);
    ///
    /// drop(item);
    /// assert_eq!(tracer.living_count(), 0);
    #[must_use]
    pub fn living_count(&self) -> usize {
        self.0.borrow().living_count
    }

    /// Returns tracing logs.
    ///
    /// # Dynamic borrowing
    ///
    /// Returned value borrows tracer dynamically. Therefore, while
    /// keeping it, actions that access to the tracer will cause panic.
    /// See [`TraceVal::clone`] or [`TraceVal::drop`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let tracer = &DropTracer::new();
    /// let vals = &mut BTreeMap::from_iter((0..3).map(|i| (i, tracer.trace(i))));
    /// let tags = &Vec::from_iter(vals.iter().map(|(_k, v)| TraceVal::tag(v)));
    /// vals.remove(&0);
    /// vals.remove(&2);
    ///
    /// let logs = tracer.logs();
    /// let logs = logs.iter().collect::<Vec<_>>();
    /// assert_eq!(logs[0], &TraceLog::new(&tags[0], true));
    /// assert_eq!(logs[1], &TraceLog::new(&tags[1], true));
    /// assert_eq!(logs[2], &TraceLog::new(&tags[2], true));
    /// assert_eq!(logs[3], &TraceLog::new(&tags[0], false));
    /// assert_eq!(logs[4], &TraceLog::new(&tags[2], false));
    /// ```
    #[must_use]
    pub fn logs(&self) -> Ref<'_, impl LogList> {
        Ref::map(self.0.borrow(), |x| &x.logs)
    }

    /// Returns states of tracing values.
    ///
    /// # Dynamic borrowing
    ///
    /// Returned value borrows tracer dynamically. Therefore, while
    /// keeping it, actions that access to the tracer will cause panic.
    /// See [`TraceVal::clone`] or [`TraceVal::drop`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let tracer = &DropTracer::new();
    /// let vals = &mut BTreeMap::from_iter((0..3).map(|i| (i, tracer.trace(i))));
    /// let tags = &Vec::from_iter(vals.iter().map(|(_k, v)| TraceVal::tag(v)));
    /// vals.remove(&0);
    /// vals.remove(&2);
    ///
    /// let map = tracer.states();
    /// assert_eq!(map.get(&tags[0]), Some(false));
    /// assert_eq!(map.get(&tags[1]), Some(true));
    /// assert_eq!(map.get(&tags[2]), Some(false));
    /// assert_eq!(map.get(&TraceTag::new()), None);
    /// ```
    #[must_use]
    pub fn states(&self) -> Ref<'_, impl StatusMap> {
        Ref::map(self.0.borrow(), |x| &x.map)
    }

    /// Returns tracing wrapper for given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tracer = &DropTracer::new();
    /// assert_eq!(tracer.living_count(), 0);
    ///
    /// let item = tracer.trace("some");
    /// assert_eq!(tracer.living_count(), 1);
    ///
    /// drop(item);
    /// assert_eq!(tracer.living_count(), 0);
    /// ```
    pub fn trace<T>(&self, value: T) -> TraceVal<T> {
        TraceVal::new(value).with_tracer(self)
    }

    /// Returns body of this instance.
    pub(crate) fn body(&self) -> Arc<RefCell<DropTracerBody>> {
        self.0.clone()
    }
}

impl DropTracerBody {
    /// Logs allocate or drop operation about given value.
    pub(crate) fn log<T>(&mut self, val: &TraceVal<T>, alloc: bool) {
        let tag = TraceVal::tag(val);
        self.logs.push(TraceLog::new_by_val(val, alloc));
        self.map.insert(tag, alloc);
        self.living_count = util::offset(self.living_count, 1, alloc);
    }
}
