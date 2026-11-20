use drop_tracer::prelude::*;

#[test]
fn iter() {
    // Arrange.
    let log1 = TraceLog::new(&TraceTag::new(), true);
    let log2 = TraceLog::new(&TraceTag::new(), false);
    let target = vec![log1, log2];
    // Act.
    let result = LogList::iter(&target);
    // Assert.
    assert!(result.eq(target.iter()));
}

#[test]
fn len() {
    // Arrange.
    let log1 = TraceLog::new(&TraceTag::new(), true);
    let log2 = TraceLog::new(&TraceTag::new(), false);
    let target = vec![log1, log2];
    // Act.
    let result = LogList::len(&target);
    // Assert.
    assert_eq!(result, 2);
}

#[test]
fn is_empty() {
    // Arrange.
    let target = Vec::<TraceLog>::new();
    // Act.
    let result = LogList::is_empty(&target);
    // Assert.
    assert!(result);
}
