//! Types.

use crate::drop_tracer::*;
use parking_lot::RwLock;
use std::sync::{Arc, Weak};

/// Reference to the drop tracer.
pub(crate) type SharedDropTracer = Arc<RwLock<DropTracerBody>>;

/// Weak reference to the drop tracer.
pub(crate) type WeakDropTracer = Weak<RwLock<DropTracerBody>>;
