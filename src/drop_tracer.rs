//! Provider of [`DropTracer`].

use crate::errors::*;
use crate::parts::*;
use crate::*;
use parking_lot::MappedRwLockReadGuard;
use parking_lot::RwLockReadGuard;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::sync::Arc;

/// Drop tracer.
///
/// See the [crate level documentation](crate) for more details.
#[derive(Clone, Debug, Default)]
pub struct DropTracer(SharedDropTracer);

/// Body of drop tracer.
#[derive(Debug, Default)]
pub(crate) struct DropTracerBody {
    /// The number of living values.
    living_count: usize,
    /// The number of dropped values.
    dropped_count: usize,
    /// Tracing logs.
    logs: Vec<TraceLog>,
    /// States (living or dropped) map for each value.
    states: BTreeMap<TraceTag, bool>,
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

    /// Returns the number of living values.
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

    /// Returns the number of dropped values.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tracer = &DropTracer::new();
    /// assert_eq!(tracer.dropped_count(), 0);
    ///
    /// let item = tracer.trace("some");
    /// assert_eq!(tracer.dropped_count(), 0);
    ///
    /// drop(item);
    /// assert_eq!(tracer.dropped_count(), 1);
    #[must_use]
    pub fn dropped_count(&self) -> usize {
        self.0.read().dropped_count
    }

    /// Returns the number of values.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tracer = &DropTracer::new();
    /// assert_eq!(tracer.total_count(), 0);
    ///
    /// let item = tracer.trace("some");
    /// assert_eq!(tracer.total_count(), 1);
    ///
    /// drop(item);
    /// assert_eq!(tracer.total_count(), 1);
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.living_count() + self.dropped_count()
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
        RwLockReadGuard::map(self.0.read(), |x| &x.states)
    }

    /// Restart the new logging.
    ///
    /// This operation clears the current logs.
    /// But it keeps the current living states.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tracer = &DropTracer::new();
    /// let item = tracer.trace("some");
    /// assert_eq!(tracer.logs().iter().count(), 1);
    ///
    /// tracer.restart();
    /// assert_eq!(tracer.logs().iter().count(), 0);
    ///
    /// drop(item);
    /// assert_eq!(tracer.logs().iter().count(), 1);
    /// ```
    pub fn restart(&self) {
        self.0.write().logs.clear();
    }

    /// Traces given new value.
    ///
    /// This is shorthand of [`trace_on`] and [`TraceVal::new`].
    ///
    /// [`trace_on`]: Self::trace_on
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
        let mut value = TraceVal::new(value);
        self.trace_on(&mut value);
        value
    }

    /// Traces given existing value.
    ///
    /// if value is already binded this tracer, no-op.
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Tracer is now reading.
    /// - Given value is already binded other tracer.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tracer = &DropTracer::new();
    /// assert_eq!(tracer.living_count(), 0);
    ///
    /// let mut val = TraceVal::new("some");
    /// tracer.trace_on(&mut val);
    /// assert_eq!(tracer.living_count(), 1);
    ///
    /// drop(val);
    /// assert_eq!(tracer.living_count(), 0);
    /// ```
    pub fn trace_on<T>(&self, value: &mut TraceVal<T>) {
        if let Some(tracer) = value.tracer() {
            if &tracer == self {
                return;
            } else {
                panic!("{}", msg::VALUE_BINDED);
            }
        }

        value.set_tracer(self);
        self.log(value, true).unwrap();
    }

    /// Creates a new instance from internal data.
    pub(crate) fn from_arc(arc: &SharedDropTracer) -> Self {
        Self(arc.clone())
    }

    /// Returns weak reference.
    pub(crate) fn weak(&self) -> WeakDropTracer {
        Arc::downgrade(&self.0)
    }

    /// Logs operation about given value.
    pub(crate) fn log<T>(&self, val: &TraceVal<T>, alloc: bool) -> Result<(), TraceError> {
        // Acquire write lock of self.
        let Some(ref mut this) = self.0.try_write() else {
            return Err(TraceError::TracerReading);
        };

        // Prepare state entry.
        let tag = TraceVal::tag(val);
        let entry = this.states.entry(tag);

        // Verify current information.
        match (alloc, &entry) {
            (true, Entry::Occupied(_)) => unreachable!(),
            (false, Entry::Vacant(_)) => unreachable!(),
            (false, Entry::Occupied(x)) if !*x.get() => return Err(TraceError::DoubleDrop),
            _ => {}
        }

        // Update and logging.
        entry.and_modify(|x| *x = alloc).or_insert(alloc);
        this.logs.reserve(alloc as usize * 2);
        this.logs.push(TraceLog::new_by_val(val, alloc));
        this.living_count = util::offset(this.living_count, 1, alloc);
        this.dropped_count += !alloc as usize;
        Ok(())
    }
}

impl Eq for DropTracer {
    // nop.
}

impl PartialEq for DropTracer {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
