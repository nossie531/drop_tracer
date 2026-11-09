//! Provider of [`TraceVal`].

use crate::errors::TraceError;
use crate::parts::*;
use crate::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
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
/// Logs are recorded if the value is bounded to the tracer.
/// Values created by [`DropTracer::trace`] are bound to the tracer.
/// On the other hand, values created by [`new`][Self::new] are not
/// bounded to any tracers. These unbounded value can be bounded
/// with [`DropTracer::trace_on`].
#[derive(Debug, Default)]
pub struct TraceVal<T> {
    /// Wrapped value.
    value: T,
    /// Trace tag.
    tag: TraceTag,
    /// Tracer body.
    tracer: Option<WeakDropTracer>,
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
    /// let tracer = &DropTracer::new();
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

    /// Returns the tracer.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let tracer = &DropTracer::new();
    /// let value = tracer.trace("sample");
    /// assert_eq!(value.tracer().as_ref(), Some(tracer));
    /// ```
    #[must_use]
    pub fn tracer(&self) -> Option<DropTracer> {
        Some(DropTracer::from_arc(&self.tracer.as_ref()?.upgrade()?))
    }

    /// Sets the tracer.
    pub(crate) fn set_tracer(&mut self, value: &DropTracer) {
        self.tracer = Some(value.weak());
    }

    /// Logs the operation about this value.
    fn log(&self, alloc: bool) -> Result<(), TraceError> {
        match self.tracer() {
            None => Ok(()),
            Some(x) => x.log(self, alloc),
        }
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
    /// (however, if panic has already occurred, double panic will not occur).
    ///
    /// # Under the hood
    ///
    /// Logging here does not require new memory allocation. Because the capacity is
    /// reserved in advance when value tracing begins. This prevents double panic flow
    /// (panic -> drop values for panic -> logging for drop -> memory allocation for
    /// logging -> memory allocation fail -> double panic).
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
