# Decisions

Design decisions to this project will be documented in this file.

## Use `parking_lot`

### Contexts

A read write lock is required to make `DropTracer` support multi threading.

### Options

- Use `parking_lot::RwLock`
- Use `std::sync::RwLock`

### Decision

Choose 1st option. Because we need `RwLockReadGuard::map` method.
As of 2026, this function is only available in the nightly with `std`.
And implementing an alternative requires self-referencing types, which
require either `unsafe` features or more crates. 