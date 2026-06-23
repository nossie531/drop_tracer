//! Provider of [`DropTracer`].

use crate::errors::*;
use crate::parts::*;
use crate::*;
use parking_lot::MappedRwLockReadGuard;
use parking_lot::RwLock;
use parking_lot::RwLockReadGuard;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Weak;

/// Drop tracer.
///
/// See the [crate level documentation](crate) for more details.
#[derive(Clone, Debug, Default)]
pub struct DropTracer(Arc<RwLock<DropTracerBody>>);

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
        self.0.read().living_count
    }

    /// Returns tracing logs.
    ///
    /// # Locking
    ///
    /// Returned value locks tracer dynamically. Therefore, while
    /// keeping it, actions that update the tracer will cause panic.
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
    pub fn logs(&self) -> MappedRwLockReadGuard<'_, impl LogList> {
        RwLockReadGuard::map(self.0.read(), |x| &x.logs)
    }

    /// Returns states of tracing values.
    ///
    /// # Locking
    ///
    /// Returned value locks tracer dynamically. Therefore, while
    /// keeping it, actions that update the tracer will cause panic.
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
    pub fn states(&self) -> MappedRwLockReadGuard<'_, impl StatusMap> {
        RwLockReadGuard::map(self.0.read(), |x| &x.map)
    }

    /// Returns tracing wrapper for given value.
    ///
    /// # Panics
    ///
    /// Panics if tracer is now reading.
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
        TraceVal::new(value).with_tracer(self).unwrap()
    }

    /// Creates a new instance from internal data.
    pub(crate) fn from_arc(arc: &Arc<RwLock<DropTracerBody>>) -> Self {
        Self(arc.clone())
    }

    /// Returns weak reference.
    pub(crate) fn weak(&self) -> Weak<RwLock<DropTracerBody>> {
        Arc::downgrade(&self.0)
    }

    /// Logs operation about given value.
    pub(crate) fn log<T>(&self, val: &TraceVal<T>, alloc: bool) -> Result<(), TraceError> {
        // Acquire write lock of self.
        let Some(ref mut this) = self.0.try_write() else {
            return Err(TraceError::TracerReading);
        };

        // Collect current informations.
        let tag = TraceVal::tag(val);
        let state = this.map.get(&tag).copied();
        let capacity = this.logs.capacity();

        // Verify current informations.
        match (alloc, state) {
            (true, Some(_)) => unreachable!(),
            (false, None) => unreachable!(),
            (false, Some(false)) => return Err(TraceError::DoubleDrop),
            _ => {}
        }

        // Reserve capacity for drop logging in the future.
        if alloc {
            let numof_val = this.map.len() + 1;
            this.logs.reserve((numof_val * 2).saturating_sub(capacity));
        }

        // Update and logging.
        this.logs.push(TraceLog::new_by_val(val, alloc));
        this.map.insert(tag, alloc);
        this.living_count = util::offset(this.living_count, 1, alloc);
        Ok(())
    }
}
