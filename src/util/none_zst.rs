//! Provider of [`NoneZst`].

use std::mem::MaybeUninit;

/// None ZST (Zero Sized Types).
#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct NoneZst(MaybeUninit<u8>);

impl Default for NoneZst {
    fn default() -> Self {
        Self(MaybeUninit::uninit())
    }
}
