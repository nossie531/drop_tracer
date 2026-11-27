use crate::for_test::*;
use drop_tracer::prelude::*;
use test_panic::prelude::*;

#[test]
fn new() {
    let result = DropTracer::new();
    assert_eq!(result.living_count(), 0);
    assert_eq!(result.dropped_count(), 0);
    assert_eq!(result.total_count(), 0);
    assert_eq!(result.logs().iter().count(), 0);
    assert_eq!(result.states().iter().count(), 0);
}

#[test]
fn living_count() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = &builder.build();
    // Act.
    let result = target.living_count();
    // Assert.
    assert_eq!(result, builder.living_count());
}

#[test]
fn dropped_count() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = &builder.build();
    // Act.
    let result = target.dropped_count();
    // Assert.
    assert_eq!(result, builder.dropped_count());
}

#[test]
fn total_count() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = &builder.build();
    // Act.
    let result = target.total_count();
    // Assert.
    assert_eq!(result, target.living_count() + target.dropped_count());
}

#[test]
fn logs() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = &builder.build();
    // Act.
    let result = target.logs();
    // Assert.
    assert!(result.iter().eq(builder.expected_logs()));
}

#[test]
fn states() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = &builder.build();
    // Act.
    let result = target.states();
    // Assert.
    let expecteds = builder.expected_states();
    let expecteds = expecteds.iter().map(|(k, v)| (k, *v));
    assert!(result.iter().eq(expecteds))
}

#[test]
fn restart() {
    // Arrange.
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = &builder.build();
    // Act.
    target.restart();
    // Assert.
    let expected_states = builder.expected_states();
    let expected_states = expected_states.iter().map(|(k, v)| (k, *v));
    assert_eq!(target.logs().iter().count(), 0);
    assert!(target.states().iter().eq(expected_states));
}

#[test]
fn trace() {
    when_normal();
    when_reading_tracer();

    fn when_normal() {
        // Arrange.
        let builder = &mut ts::TracerBuilder::new_normal();
        let target = &builder.build();
        // Act.
        let result = target.trace(ts::SOME);
        // Assert.
        let new_log = TraceLog::new(&TraceVal::tag(&result), true);
        let logs_old = builder.expected_logs().iter().cloned();
        let logs_asis = &Vec::from_iter(target.logs().iter().cloned());
        let logs_tobe = &Vec::from_iter(logs_old.chain([new_log]));
        assert_eq!(*result, ts::SOME);
        assert_eq!(TraceVal::is_bomb(&result), false);
        assert_eq!(logs_asis, logs_tobe);
    }

    fn when_reading_tracer() {
        // Arrange.
        let builder = &mut ts::TracerBuilder::new_normal();
        let target = &builder.build();
        let logs = target.logs();
        // Act.
        let result = test_panic(|| target.trace(ts::SOME));
        // Assert.
        assert!(result.is_panic());
        // Clean up.
        std::mem::drop(logs);
    }
}

#[test]
fn trace_bomb() {
    when_normal();
    when_reading_tracer();

    fn when_normal() {
        // Arrange.
        let builder = &mut ts::TracerBuilder::new_normal();
        let target = &builder.build();
        // Act.
        let result = target.trace_bomb(ts::SOME, true);
        // Assert.
        let new_log = TraceLog::new(&TraceVal::tag(&result), true);
        let logs_old = builder.expected_logs().iter().cloned();
        let logs_asis = &Vec::from_iter(target.logs().iter().cloned());
        let logs_tobe = &Vec::from_iter(logs_old.chain([new_log]));
        assert_eq!(*result, ts::SOME);
        assert_eq!(TraceVal::is_bomb(&result), true);
        assert_eq!(logs_asis, logs_tobe);
        // Clean up.
        std::mem::forget(result);
    }

    fn when_reading_tracer() {
        // Arrange.
        let builder = &mut ts::TracerBuilder::new_normal();
        let target = &builder.build();
        let logs = target.logs();
        // Act.
        let result = test_panic(|| target.trace_bomb(ts::SOME, false));
        // Assert.
        assert!(result.is_panic());
        // Clean up.
        std::mem::drop(logs);
    }
}

#[test]
fn trace_on() {
    when_normal();
    when_reading_tracer();
    when_dup();
    when_value_of_others();

    fn when_normal() {
        // Arrange.
        let builder = &mut ts::TracerBuilder::new_normal();
        let target = &builder.build();
        let value = &TraceVal::new(ts::SOME);
        // Act.
        target.trace_on(value);
        // Assert.
        let new_log = TraceLog::new(&TraceVal::tag(&value), true);
        let logs_old = builder.expected_logs().iter().cloned();
        let logs_asis = &Vec::from_iter(target.logs().iter().cloned());
        let logs_tobe = &Vec::from_iter(logs_old.chain([new_log]));
        assert_eq!(logs_asis, logs_tobe);
    }

    fn when_reading_tracer() {
        // Arrange.
        let builder = &mut ts::TracerBuilder::new_normal();
        let target = &builder.build();
        let logs = target.logs();
        // Act.
        let result = test_panic(|| target.trace_on(&TraceVal::new(ts::SOME)));
        // Assert.
        assert!(result.is_panic());
        // Clean up.
        std::mem::drop(logs);
    }

    fn when_dup() {
        // Arrange.
        let builder = &mut ts::TracerBuilder::new_normal();
        let target = &builder.build();
        let value = &target.trace(ts::SOME);
        // Act.
        target.trace_on(value);
        // Assert.
        let new_log = TraceLog::new(&TraceVal::tag(&value), true);
        let logs_old = builder.expected_logs().iter().cloned();
        let logs_asis = &Vec::from_iter(target.logs().iter().cloned());
        let logs_tobe = &Vec::from_iter(logs_old.chain([new_log]));
        assert_eq!(logs_asis, logs_tobe);
    }

    fn when_value_of_others() {
        // Arrange.
        let builder1 = &mut ts::TracerBuilder::new_normal();
        let builder2 = &mut ts::TracerBuilder::new_normal();
        let target = &builder1.build();
        let other = &builder2.build();
        let value = &other.trace(ts::SOME);
        // Act.
        let result = test_panic(|| target.trace_on(value));
        // Assert.
        assert!(result.is_panic());
    }
}

#[test]
fn clone() {
    let builder = &mut ts::TracerBuilder::new_normal();
    let target = &builder.build();
    let result = target.clone();
    assert_eq!(&result, target);
}
