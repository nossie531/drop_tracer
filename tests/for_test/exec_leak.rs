use super::Chain;
use drop_tracer::DropTracer;

pub fn exec_no_leak(t: &mut DropTracer) {
    let x = Chain::new(t.new_item());
    let y = Chain::new(t.new_item());
    x.borrow_mut().link = Some(y.clone());
}

pub fn exec_leak(t: &mut DropTracer) {
    let x = Chain::new(t.new_item());
    let y = Chain::new(t.new_item());
    x.borrow_mut().link = Some(y.clone());
    y.borrow_mut().link = Some(x.clone());
}
