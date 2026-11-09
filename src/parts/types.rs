//! Types.

use crate::drop_tracer::DropTracerBody;
use parking_lot::RwLock;
use std::panic::AssertUnwindSafe;
use std::sync::{Arc, Weak};

/// Reference to the drop tracer.
pub(crate) type SharedDropTracer = Arc<AssertUnwindSafe<RwLock<DropTracerBody>>>;

/// Weak reference to the drop tracer.
pub(crate) type WeakDropTracer = Weak<AssertUnwindSafe<RwLock<DropTracerBody>>>;
