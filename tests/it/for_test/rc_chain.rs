use drop_tracer::prelude::TraceVal;
use std::cell::RefCell;
use std::rc::Rc;

pub struct RcChain {
    pub next: Option<Rc<RefCell<Self>>>,
    pub _tag: Option<TraceVal<i32>>,
}

impl RcChain {
    pub fn new(tag: Option<TraceVal<i32>>) -> Rc<RefCell<Self>> {
        let ret = Self {
            next: None,
            _tag: tag,
        };
        Rc::new(RefCell::new(ret))
    }

    pub fn new_without_leak(tag: TraceVal<i32>) -> Rc<RefCell<Self>> {
        Self::new(Some(tag))
    }

    pub fn new_with_leak(tag: TraceVal<i32>) -> Rc<RefCell<Self>> {
        let x = Self::new(Some(tag));
        let y = Self::new(None);
        x.borrow_mut().next = Some(y.clone());
        y.borrow_mut().next = Some(x.clone());
        x
    }
}
