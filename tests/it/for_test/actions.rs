use crate::for_test::RcChain;
use drop_tracer::DropTracer;

pub fn without_leak(tracer: &mut DropTracer) {
    RcChain::new_without_leak(tracer.trace(42));
}

pub fn with_leak(tracer: &mut DropTracer) {
    RcChain::new_with_leak(tracer.trace(42));
}
