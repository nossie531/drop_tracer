use crate::for_test::*;
use drop_tracer::prelude::*;

#[test]
fn new() {
    let result = DropTracer::new();
    assert_eq!(result.living_count(), 0);
    assert_eq!(result.logs().iter().count(), 0);
}

#[test]
fn living_count() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = builder.take();
    // Act.
    let result = target.living_count();
    // Assert.
    assert_eq!(result, builder.living_count());
}

#[test]
fn logs() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = builder.take();
    // Act.
    let result = target.logs();
    // Assert.
    assert!(result.iter().eq(builder.expected_logs()));
}

#[test]
fn status() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = builder.take();
    // Act.
    let result = target.states();
    // Assert.
    let expecteds = builder.expected_states();
    let expecteds = expecteds.iter().map(|(k, v)| (k, *v));
    assert!(result.iter().eq(expecteds))
}

#[test]
fn trace() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new();
    let target = builder.take();
    // Act.
    let result = target.trace(ts::SOME);
    // Assert.
    assert_eq!(*result, ts::SOME);
}
