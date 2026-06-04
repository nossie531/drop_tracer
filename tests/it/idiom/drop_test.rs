use crate::for_test::*;
use drop_tracer::prelude::*;
use test_panic::prelude::*;

#[test]
fn run() {
    when_no_leak();
    when_leak();

    fn when_no_leak() {
        let action = actions::without_leak;
        drop_test::run(action);
    }

    fn when_leak() {
        let action = actions::with_leak;
        let result = test_panic(|| drop_test::run(action));
        assert!(result.is_panic());
    }
}

#[test]
fn try_run() {
    when_no_leak();
    when_leak();

    fn when_no_leak() {
        let action = actions::without_leak;
        let result = drop_test::try_run(action);
        assert!(result.is_ok());
    }

    fn when_leak() {
        let action = actions::with_leak;
        let result = drop_test::try_run(action);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().count(), 1);
    }
}
