mod for_test;

use drop_tracer::DropTracer;
use for_test::{exec_leak, exec_no_leak};
use test_panic::test_panic;

#[test]
fn new() {
    let result = DropTracer::new();
    assert_eq!(result.count(), 0);
}

#[test]
fn test_drop() {
    with_no_leak();
    with_leak();

    fn with_no_leak() {
        DropTracer::test_drop(exec_no_leak);
    }

    fn with_leak() {
        let result = test_panic(|| DropTracer::test_drop(exec_leak));
        assert!(result.is_panic());
    }
}

#[test]
fn try_drop() {
    with_no_leak();
    with_leak();

    fn with_no_leak() {
        let result = DropTracer::try_drop(exec_no_leak);
        assert!(result.is_ok());
    }

    fn with_leak() {
        let result = DropTracer::try_drop(exec_leak);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().count(), 2);
    }
}

#[test]
fn new_item() {
    let target = DropTracer::new();
    let _result = target.new_item();
    assert_eq!(target.count(), 1);
}
