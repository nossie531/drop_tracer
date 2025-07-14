drop_tracer
===

Simple memory leak detector.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate performs simple memory leak detection. This is intended to check
local memory leaks in small crates. This is simple to use, just generate
items for tracing, embed them in target to be managed, and check later to see
if they have been dropped.

(`#[global_allocator]` attribute is not necessary).

## Examples

Example using `try_drop` (see manual for other options).

```rust
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

## Versions

See [CHANGELOG](CHANGELOG.md).
