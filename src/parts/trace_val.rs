//! Provider of [`TraceVal`].

use crate::errors::TraceError;
use crate::prelude::*;
use crate::*;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, Weak};
use std::thread;

/// Tracing value wrapper.
///
/// # Wrapping
///
/// This wrapper type can be treated almost like original type.
/// For example, [`Deref`] simply returns the wrapped value.
/// Also, basic traits such as [`Clone`], [`Default`], [`Eq`],
/// [`Ord`], and [`Hash`] follow the wrapped type.
///
/// # Tracing
///
/// Values created by [`DropTracer::trace`] are bound to the tracer.
/// On the other hand, values created by [`new`][Self::new] are not
/// bound to a tracers. Then, when a value bound to the tracer is
/// dropped, a log is recorded in the tracer.
#[derive(Debug, Default)]
pub struct TraceVal<T> {
    /// Wrapped value.
    value: T,
    /// Trace tag.
    tag: TraceTag,
    /// Tracer body.
    tracer: Option<Weak<RefCell<DropTracerBody>>>,
}

impl<T> TraceVal<T> {
    /// Creates a new instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let result = TraceVal::new("sample");
    /// assert_eq!(*result, "sample");
    /// ```
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            value,
            tag: TraceTag::new(),
            tracer: None,
        }
    }

    /// Returns trace tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tracer = &mut DropTracer::new();
    /// let value = tracer.trace("sample");
    /// let tag = TraceVal::tag(&value);
    /// drop(value);
    ///
    /// let logs = &mut tracer.logs();
    /// let logs = &mut logs.iter().collect::<Vec<_>>();
    /// assert_eq!(logs[0], &TraceLog::new(&tag, true));
    /// assert_eq!(logs[1], &TraceLog::new(&tag, false));
    /// ```
    #[must_use]
    pub fn tag(this: &Self) -> TraceTag {
        this.tag.clone()
    }

    /// Creates a new instance with new tracer.
    pub(crate) fn with_tracer(mut self, value: &DropTracer) -> Self {
        debug_assert!(self.tracer.is_none());

        // Logs the new value to the tracer.
        let tracer = &value.body();
        tracer.borrow_mut().log(&self, true).unwrap();

        // Returns the new instance.
        self.tracer = Some(Arc::downgrade(tracer));
        self
    }

    /// Logs the allocation or drop of this value.
    fn log(&self, alloc: bool) -> Result<(), TraceError> {
        let Some(tracer) = self.tracer.as_ref() else {
            return Ok(());
        };

        let Some(tracer) = Weak::upgrade(tracer) else {
            return Ok(());
        };

        let Some(ref mut tracer) = tracer.try_borrow_mut().ok() else {
            return Err(TraceError::TracerBorrowed);
        };

        tracer.log(self, alloc)
    }
}

impl<T> Deref for TraceVal<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Clone for TraceVal<T>
where
    T: Clone,
{
    /// Returns duplicate of traced value with logging.
    ///
    /// # Clone flag
    ///
    /// Tag of returned value is marked as clone. This can be checked by
    /// [`TraceTag::for_clone`]. This allows you to determine whether the
    /// target being traced is the original or its clone.
    ///
    /// # Panics
    ///
    /// Panics if related tracer is borrowed already.
    fn clone(&self) -> Self {
        let ret = Self {
            value: self.value.clone(),
            tag: TraceTag::new_for_clone(),
            tracer: self.tracer.clone(),
        };

        let log_result = ret.log(true);
        if let Err(error) = log_result {
            error.panic();
        }

        ret
    }
}

impl<T> Drop for TraceVal<T> {
    /// Execute the destructor with logging.
    ///
    /// # Panics
    ///
    /// Panics if related tracer is borrowed already or double drop is detected
    /// (however, if panic has already occurred, doubl panic will not occur).
    fn drop(&mut self) {
        let log_result = self.log(false);
        if let Err(error) = log_result
            && !thread::panicking()
        {
            error.panic();
        }
    }
}

impl<T> Eq for TraceVal<T>
where
    T: Eq,
{
    // nop.
}

impl<T> Hash for TraceVal<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> Ord for TraceVal<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> PartialEq for TraceVal<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T> PartialOrd for TraceVal<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
