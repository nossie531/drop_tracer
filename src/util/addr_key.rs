//! Provider of [`AddrKey`].

use crate::util::*;
use crate::*;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// The key based on an internal heap address.
#[derive(Clone, Default)]
pub(crate) struct AddrKey(Arc<NoneZst>);

impl AddrKey {
    /// Creates a new instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the address of internal key.
    fn addr(&self) -> usize {
        Arc::as_ptr(&self.0) as usize
    }
}

impl Debug for AddrKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AddrKey").field(&self.addr()).finish()
    }
}

impl Eq for AddrKey {
    // nop.
}

impl Hash for AddrKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr().hash(state);
    }
}

impl Ord for AddrKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let ptr_self = Arc::as_ptr(&self.0) as *const _;
        let ptr_other = Arc::as_ptr(&other.0) as *const _;
        util::cmp_ptr(ptr_self, ptr_other)
    }
}

impl PartialEq for AddrKey {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for AddrKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
