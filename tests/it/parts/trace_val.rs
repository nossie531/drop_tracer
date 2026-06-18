use crate::for_test::*;
use drop_tracer::prelude::*;
use std::{cmp::Ordering, mem, ops::Deref, ptr};
use test_panic::prelude::*;

#[test]
fn new() {
    let result = TraceVal::new(ts::SOME);
    assert_eq!(*result, ts::SOME);
    assert_eq!(TraceVal::tag(&result).for_clone(), false);
}

#[test]
fn tag() {
    // Arrange.
    let target1 = TraceVal::new(ts::SOME);
    let target2 = TraceVal::new(ts::SOME);
    // Act.
    let result1 = TraceVal::tag(&target1);
    let result2 = TraceVal::tag(&target2);
    // Assert.
    assert_eq!(result1, TraceVal::tag(&target1));
    assert_eq!(result2, TraceVal::tag(&target2));
    assert_ne!(result1, result2);
}

#[test]
fn deref() {
    // Arrange.
    let target = TraceVal::new(ts::SOME);
    // Act.
    let result = target.deref();
    // Assert.
    assert_eq!(result, &ts::SOME);
}

#[test]
fn clone() {
    when_without_tracer();
    when_with_tracer();
    when_reading_tracer();

    fn when_without_tracer() {
        // Arrange.
        let target = TraceVal::new(ts::SOME);
        // Act.
        let result = target.clone();
        // Assert.
        assert_eq!(result, target);
    }

    fn when_with_tracer() {
        // Arrange.
        let tracer = &mut DropTracer::new();
        let target = tracer.trace(ts::SOME);
        // Act.
        let result = target.clone();
        // Assert.
        let logs = tracer.logs();
        let last_log_asis = logs.iter().last().unwrap();
        let last_log_tobe = &TraceLog::new(&TraceVal::tag(&result), true);
        assert_eq!(*result, *target);
        assert_eq!(TraceVal::tag(&result).for_clone(), true);
        assert_ne!(TraceVal::tag(&result), TraceVal::tag(&target));
        assert_eq!(last_log_asis, last_log_tobe);
    }

    fn when_reading_tracer() {
        // Arrange.
        let tracer = &mut DropTracer::new();
        let target = tracer.trace(ts::SOME);
        let logs = tracer.logs();
        // Act.
        let result = test_panic(|| target.clone());
        // Assert.
        assert!(result.is_panic());
        // Clean up.
        mem::drop(logs);
    }
}

#[test]
fn drop() {
    when_normal();
    when_reading_tracer();
    when_double_drop();

    fn when_normal() {
        // Arrange.
        let tracer = &mut DropTracer::new();
        let target = tracer.trace(ts::SOME);
        let tag = TraceVal::tag(&target);
        // Act.
        mem::drop(target);
        // Assert.
        let states = tracer.states();
        assert_eq!(states.get(&tag), Some(false));
    }

    fn when_reading_tracer() {
        // Arrange.
        let tracer = &mut DropTracer::new();
        let target = tracer.trace(ts::SOME);
        let logs = tracer.logs();
        // Act.
        let result = test_panic(|| mem::drop(target));
        // Assert.
        assert!(result.is_panic());
        // Clean up.
        mem::drop(logs);
    }

    fn when_double_drop() {
        // Arrange.
        let builder = &mut ts::TracerBuilder::new_normal();
        let target = builder.take();
        let mut val = target.trace(ts::SOME);
        unsafe { ptr::drop_in_place((&mut val) as *mut _) }
        // Act.
        let result = test_panic(|| mem::drop(val));
        // Assert.
        assert!(result.is_panic());
    }
}

#[test]
fn hash() {
    // Arrange.
    let target = TraceVal::new(ts::SOME);
    // Act.
    let result = tu::get_hash(&target);
    // Assert.
    assert_eq!(result, tu::get_hash(&TraceVal::new(ts::SOME)));
    assert_ne!(result, tu::get_hash(&TraceVal::new(ts::SOME + 1)));
}

#[test]
fn cmp() {
    // Arrange.
    let target = TraceVal::new(ts::SOME);
    // Act.
    let result1 = target.cmp(&TraceVal::new(ts::SOME));
    let result2 = target.cmp(&TraceVal::new(ts::SOME - 1));
    let result3 = target.cmp(&TraceVal::new(ts::SOME + 1));
    // Assert.
    assert_eq!(result1, Ordering::Equal);
    assert_eq!(result2, Ordering::Greater);
    assert_eq!(result3, Ordering::Less);
}

#[test]
fn partial_eq() {
    // Arrange.
    let target = TraceVal::new(ts::SOME);
    // Act.
    let result1 = target.eq(&TraceVal::new(ts::SOME));
    let result2 = target.eq(&TraceVal::new(ts::SOME + 1));
    // Assert.
    assert_eq!(result1, true);
    assert_eq!(result2, false);
}

#[test]
fn partial_cmp() {
    // Arrange.
    let target = TraceVal::new(ts::SOME_FLOAT);
    // Act.
    let result1 = target.partial_cmp(&TraceVal::new(ts::SOME_FLOAT));
    let result2 = target.partial_cmp(&TraceVal::new(ts::SOME_FLOAT - 1.0));
    let result3 = target.partial_cmp(&TraceVal::new(ts::SOME_FLOAT + 1.0));
    let result4 = target.partial_cmp(&TraceVal::new(ts::ValFloat::NAN));
    // Assert.
    assert_eq!(result1, Some(Ordering::Equal));
    assert_eq!(result2, Some(Ordering::Greater));
    assert_eq!(result3, Some(Ordering::Less));
    assert_eq!(result4, None);
}
