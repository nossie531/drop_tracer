use drop_tracer::DropItem;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Chain {
    pub link: Option<Rc<RefCell<Self>>>,
    pub _d: DropItem,
}

impl Chain {
    pub fn new(d: DropItem) -> Rc<RefCell<Self>> {
        let result = Self { link: None, _d: d };
        Rc::new(RefCell::new(result))
    }
}
