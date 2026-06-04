use drop_tracer::prelude::*;
use std::{array, collections::BTreeMap};

#[test]
fn get() {
    // Arrange.
    let tags = array::from_fn::<_, 2, _>(|_| TraceTag::new());
    let target = BTreeMap::from_iter([(tags[0].clone(), false), (tags[1].clone(), true)]);
    // Act.
    let result1 = StatusMap::get(&target, &tags[0]);
    let result2 = StatusMap::get(&target, &tags[1]);
    let result3 = StatusMap::get(&target, &TraceTag::new());
    // Assert.
    assert_eq!(result1, Some(false));
    assert_eq!(result2, Some(true));
    assert_eq!(result3, None);
}

#[test]
fn iter() {
    // Arrange.
    let tags = array::from_fn::<_, 2, _>(|_| TraceTag::new());
    let target = BTreeMap::from_iter([(tags[0].clone(), false), (tags[1].clone(), true)]);
    // Act.
    let asis = StatusMap::iter(&target);
    // Assert.
    let tobe = target.iter().map(|(k, v)| (k, *v));
    assert!(asis.eq(tobe));
}
