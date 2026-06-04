use crate::for_test::*;
use drop_tracer::prelude::*;

#[test]
fn new() {
    let result = TraceTag::new();
    assert_eq!(result.for_clone(), false);
}

#[test]
fn new_for_clone() {
    let tag = TraceTag::new_for_clone();
    assert_eq!(tag.for_clone(), true);
}

#[test]
fn for_clone() {
    let target1 = TraceTag::new();
    let target2 = TraceTag::new_for_clone();
    assert_eq!(target1.for_clone(), false);
    assert_eq!(target2.for_clone(), true);
}

#[test]
fn fmt() {
    let target = TraceTag::new();
    let _ = format!("{:?}", target);
}

#[test]
fn hash() {
    let target = TraceTag::new();
    let result = tu::get_hash(&target);
    assert_eq!(result, tu::get_hash(&target));
    assert_ne!(result, tu::get_hash(&TraceTag::new()));
}
