# drop_tracer

Instance drop tracer.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate helps to trace the execution of `Drop::drop`.\
It is primarily intended for use in testing.

## Core items

The core item of this crate is `DropTracer`.\
`DropTracer` creates tracing values by `trace` method.\
These values' allocations and drops are logged by the tracer.

## Examples

### By helper

`drop_test` is helper for local memory leak check.\
Following code detects memory leak by cyclic `Rc`.

```rust
use std::cell::RefCell;
use std::rc::Rc;
use drop_tracer::prelude::*;

let result = drop_test::try_run(|t| {
    let x = RcChain::new(t.trace("item1"));
    let y = RcChain::new(t.trace("item2"));
    x.borrow_mut().next = Some(y.clone());
    y.borrow_mut().next = Some(x.clone());
});

assert_eq!(result.unwrap_err().count(), 2);

struct RcChain {
    next: Option<Rc<RefCell<Self>>>,
    _val: TraceVal<&'static str>,
}

impl RcChain {
    pub fn new(val: TraceVal<&'static str>) -> Rc<RefCell<Self>> {
        let result = Self { next: None, _val: val };
        Rc::new(RefCell::new(result))
    }
}
```

### By manual

Following code trace values in vector.

```rust
use drop_tracer::prelude::*;

// Arrange tracer and data structure.
let tracer = &DropTracer::new();
let data = &mut Vec::new();

// Create values for trace.
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

// Verify tracer.
let logs = tracer.logs();
assert_eq!(tracer.living_count(), 2);
assert!(logs.iter().eq([
    &TraceLog::new(&tag1, true),
    &TraceLog::new(&tag2, true),
    &TraceLog::new(&tag3, true),
    &TraceLog::new(&tag1, false),
]));
```

### Bomb panic

The following code tests panic on drop.

Note: Panic on drop is not recommended.\
But, robust collections consider element drop panic.

```rust
use drop_tracer::prelude::*;
use std::panic::{self, AssertUnwindSafe};

// Arrange tracer.
let tracer = &DropTracer::new();

// Create values with bomb.
let val1 = tracer.trace(1);
let val2 = tracer.trace_bomb(2, true);
let val3 = tracer.trace(3);
let data = &mut vec![val1, val2, val3];

// Do some action.
panic::catch_unwind(AssertUnwindSafe(|| {
    data.clear();
}));

// Verify tracer.
let logs = tracer.logs();
assert_eq!(tracer.living_count(), 0);
```

## History

See [CHANGELOG](CHANGELOG.md).
