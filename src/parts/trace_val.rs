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
/// This wrapper type can be treated like original type. However, this type
/// has some original fields like the bomb flag. So, be careful that those
/// fields are ignored at comparing by [`Eq`], [`Ord`] and [`Hash`].
///
/// # Tracing
///
/// Logs are recorded if the value is bounded to the tracer.
/// Values created by [`DropTracer::trace`] are bound to the tracer.
/// On the other hand, values created by [`new`][Self::new] are not
/// bounded to any tracers. These unbounded value can be bounded
/// with [`DropTracer::trace_on`].
///
/// # Bomb panic
///
/// Bomb panics are occured if the the value is traced and bomb flag is set.
/// (see ["Tracing"](#Tracing) about tracing rule).
#[derive(Debug, Default)]
pub struct TraceVal<T> {
    /// Wrapped value.
    value: T,
    /// Bomb flag.
    bomb: bool,
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
            bomb: false,
            tag: TraceTag::new(),
            tracer: None,
        }
    }

    /// Creates a new bomb instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let val = TraceVal::new_bomb("sample", true);
    /// assert!(TraceVal::is_bomb(&val));
    /// ```
    #[must_use]
    pub fn new_bomb(value: T, bomb: bool) -> Self {
        Self::with_bomb(Self::new(value), bomb)
    }

    /// Creates a new instance with the given bomb flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let val = TraceVal::new("sample");
    /// let val = TraceVal::with_bomb(val, true);
    /// assert!(TraceVal::is_bomb(&val));
    /// ```
    #[must_use]
    pub fn with_bomb(mut this: Self, value: bool) -> Self {
        this.bomb = value;
        this
    }

    /// Returns `true` if this is bomb.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let val = TraceVal::new_bomb("sample", true);
    /// assert!(TraceVal::is_bomb(&val));
    /// ```
    #[must_use]
    pub fn is_bomb(this: &Self) -> bool {
        this.bomb
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
    /// let value = &tracer.trace("sample");
    /// assert_eq!(TraceVal::tracer(value).as_ref(), Some(tracer));
    /// ```
    #[must_use]
    pub fn tracer(this: &Self) -> Option<DropTracer> {
        Some(DropTracer::from_arc(&this.tracer.as_ref()?.upgrade()?))
    }

    /// Sets the tracer.
    pub(crate) fn set_tracer(this: &mut Self, value: &DropTracer) {
        this.tracer = Some(value.weak());
    }

    /// Logs the operation about this value.
    fn log(&self, alloc: bool) -> Result<(), TraceError> {
        match Self::tracer(self) {
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
            bomb: self.bomb,
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
    /// Panics in the following cases.
    ///
    /// - Bomb flag and tracer is setted.
    /// - Related tracer is borrowed already.
    /// - Double drop is detected.
    ///
    /// If panic has already occurred, double panic
    /// will not occur to avoid process aborting.
    ///
    /// # Under the hood
    ///
    /// Logging here does not require new memory allocation. Because the capacity is
    /// reserved in advance when value tracing begins. This prevents double panic flow
    /// (panic -> drop values for panic -> logging for drop -> memory allocation for
    /// logging -> memory allocation fail -> double panic).
    fn drop(&mut self) {
        let log_result = self.log(false);

        if thread::panicking() {
            return;
        }

        if let Err(error) = log_result {
            error.panic();
        }

        if self.bomb && Self::tracer(self).is_some() {
            panic!("{}", msg::BOMB_EXPLODED);
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
