mod drop_tracer;
mod for_test;
mod idiom;
mod parts;

#[test]
fn playground() {
    use ::drop_tracer::prelude::*;

    // Arrange tracer and data structure.
    let tracer = DropTracer::new();
    let data = &mut Vec::new();

    // Creates values for trace.
    let val1 = tracer.trace(1);
    let val2 = tracer.trace(2);
    let val3 = tracer.trace(3);

    // Save tags for detail trace.
    let tag1 = TraceVal::tag(&val1);
    let tag2 = TraceVal::tag(&val2);
    let tag3 = TraceVal::tag(&val3);

    // Do some action.
    data.extend([val1, val2, val3]);
    data.remove(0);

    // Veryfy tracer.
    let logs = tracer.logs();
    assert_eq!(tracer.living_count(), 2);
    assert!(logs.iter().eq([
        &TraceLog::new(&tag1, true),
        &TraceLog::new(&tag2, true),
        &TraceLog::new(&tag3, true),
        &TraceLog::new(&tag1, false),
    ]));
}
