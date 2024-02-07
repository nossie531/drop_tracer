/*! Simple memory leak detector.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This crate has three main entry points.

* [`DropTracer::test_drop`] -
  For rough test. Panic if memory leak is detected.
* [`DropTracer::try_drop`] -
  For detail test. Receive error information if memory leak is detected.
* [`DropTracer::new`] -
  For manual control. On yourself, generate tracers and check the drop status.

# Examples

```
# use std::cell::RefCell;
# use std::rc::Rc;
# use drop_tracer::DropItem;
# use drop_tracer::DropTracer;
let result = DropTracer::try_drop(|t| {
    let x = Chain::new(t.new_item());
    let y = Chain::new(t.new_item());
    x.borrow_mut().link = Some(y.clone());
    y.borrow_mut().link = Some(x.clone());
});

assert_eq!(result.unwrap_err().count(), 2);

struct Chain {
    link: Option<Rc<RefCell<Self>>>,
    _d: DropItem,
}

impl Chain {
    pub fn new(d: DropItem) -> Rc<RefCell<Self>> {
        let result = Self { link: None, _d: d };
        Rc::new(RefCell::new(result))
    }
}
```
*/

mod drop_item;
mod drop_tracer;
mod leak_error;

pub use drop_item::*;
pub use drop_tracer::*;
pub use leak_error::*;
