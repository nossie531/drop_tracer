pub(crate) use tracer_builder::*;

pub(crate) type Val = i32;
pub(crate) type ValFloat = f32;

pub(crate) const SOME: Val = 42;
pub(crate) const SOME_FLOAT: f32 = 42.0;

mod tracer_builder;
