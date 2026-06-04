use drop_tracer::prelude::*;

#[test]
fn new() {
    // Arrange.
    let tag = &TraceTag::new();
    // Act.
    let result = &TraceLog::new(tag, true);
    // Assert.
    assert_eq!(result.tag(), tag);
    assert_eq!(result.is_alloc(), true);
}

#[test]
fn new_by_val() {
    // Arrange.
    let val = &TraceVal::new(42);
    // Act.
    let result = &TraceLog::new_by_val(val, true);
    // Assert.
    assert_eq!(result.tag(), &TraceVal::tag(val));
    assert_eq!(result.is_alloc(), true);
}

#[test]
fn tag() {
    // Arrange.
    let tag = &TraceTag::new();
    // Act.
    let result = &TraceLog::new(tag, true);
    // Assert.
    assert_eq!(result.tag(), tag);
}

#[test]
fn is_alloc() {
    // Arrange.
    let tag = &TraceTag::new();
    let target1 = &TraceLog::new(tag, true);
    let target2 = &TraceLog::new(tag, false);
    // Act.
    let result1 = target1.is_alloc();
    let result2 = target2.is_alloc();
    // Assert.
    assert_eq!(result1, true);
    assert_eq!(result2, false);
}

#[test]
fn is_cloned() {
    // Arrange.
    let target1 = &TraceLog::new(&TraceTag::new(), true);
    let target2 = &TraceLog::new(&TraceTag::new_for_clone(), true);
    // Act.
    let result1 = target1.is_cloned();
    let result2 = target2.is_cloned();
    // Assert.
    assert_eq!(result1, false);
    assert_eq!(result2, true);
}
