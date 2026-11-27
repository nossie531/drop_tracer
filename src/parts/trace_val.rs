//! Provider of [`TraceVal`].

use crate::errors::*;
use crate::parts::*;
use crate::util::*;
use crate::*;
use parking_lot::RwLock;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
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
/// Value has following three bomb modes.
///
/// - None: No panic when dropped.
/// - Auto: Always panic when dropped (Easy setting).
/// - Normal: Only panic when traced and dropped (Easy releasing).
#[derive(Debug, Default)]
pub struct TraceVal<T> {
    /// Wrapped value.
    value: T,
    /// Bomb flag.
    bomb: RwLock<BombMode>,
    /// Trace tag.
    tag: TraceTag,
    /// Tracer body.
    tracer: AsNormal<RwLock<Option<WeakDropTracer>>>,
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
            bomb: RwLock::new(BombMode::None),
            tag: TraceTag::new(),
            tracer: AsNormal::new(RwLock::new(None)),
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

    /// Creates a new bomb instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let val = TraceVal::new_auto_bomb("sample", true);
    /// assert!(TraceVal::is_auto_bomb(&val));
    /// std::mem::forget(val);
    /// ```
    #[must_use]
    pub fn new_auto_bomb(value: T, bomb: bool) -> Self {
        Self::with_auto_bomb(Self::new(value), bomb)
    }

    /// Creates a new instance with bomb flag.
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
        let mode = if value { BombMode::Normal } else { BombMode::None };
        this.bomb = RwLock::new(mode);
        this
    }

    /// Creates a new instance with auto bomb flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let val = TraceVal::new("sample");
    /// let val = TraceVal::with_auto_bomb(val, true);
    /// assert!(TraceVal::is_auto_bomb(&val));
    /// std::mem::forget(val);
    /// ```
    #[must_use]
    pub fn with_auto_bomb(mut this: Self, value: bool) -> Self {
        let mode = if value { BombMode::Auto } else { BombMode::None };
        this.bomb = RwLock::new(mode);
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
        this.bomb.read().is_bomb()
    }

    /// Returns `true` if this is auto bomb.
    ///
    /// # Examples
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let val = TraceVal::new_auto_bomb("sample", true);
    /// assert!(TraceVal::is_auto_bomb(&val));
    /// std::mem::forget(val);
    /// ```
    #[must_use]
    pub fn is_auto_bomb(this: &Self) -> bool {
        this.bomb.read().is_auto_bomb()
    }

    /// Sets the bomb flag.
    ///
    /// # Example
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let val = TraceVal::new("sample");
    /// TraceVal::set_bomb(&val, true);
    /// assert!(TraceVal::is_bomb(&val));
    /// ```
    pub fn set_bomb(this: &Self, value: bool) {
        let mode = if value { BombMode::Normal } else { BombMode::None };
        *this.bomb.write() = mode;
    }

    /// Sets the auto bomb flag.
    ///
    /// # Example
    ///
    /// ```
    /// use drop_tracer::prelude::*;
    ///
    /// let val = TraceVal::new("sample");
    /// TraceVal::set_bomb(&val, true);
    /// assert!(TraceVal::is_bomb(&val));
    /// std::mem::forget(val);
    /// ```
    pub fn set_auto_bomb(this: &Self, value: bool) {
        let mode = if value { BombMode::Auto } else { BombMode::None };
        *this.bomb.write() = mode;
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
        let tracer = this.tracer.read().as_ref()?.upgrade()?;
        Some(DropTracer::from_arc(&tracer))
    }

    /// Sets the tracer.
    pub(crate) fn set_tracer(this: &Self, value: &DropTracer) {
        *this.tracer.write() = Some(value.weak());
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

impl<T> DerefMut for TraceVal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
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
        let tracer = self.tracer.read().as_ref().map(WeakDropTracer::clone);

        let ret = Self {
            value: self.value.clone(),
            bomb: RwLock::new(*self.bomb.read()),
            tag: TraceTag::new_for_clone(),
            tracer: AsNormal::new(RwLock::new(tracer)),
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

        if self.bomb.read().is_explodes(Self::tracer(self).is_some()) {
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
