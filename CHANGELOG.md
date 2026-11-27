# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

No items.

## [0.9.0] - 2026-11-28

### Added

- `TraceVal::new_auto_bomb`
- `TraceVal::with_auto_bomb`
- `TraceVal::is_auto_bomb`
- `TraceVal::set_bomb`
- `TraceVal::set_auto_bomb`

### Changed

- `DropTracer::trace_on` arguments type

## [0.8.0] - 2026-11-25

### Added

- `DerefMut` for `TraceVal`

### Changed

- Polish documentations.

## [0.7.0] - 2026-11-22

### Added

- `DropTracer::restart`
- `DropTracer::trace_bomb`
- `TraceVal::new_bomb`
- `TraceVal::with_bomb`
- `TraceVal::is_bomb`
- `UnwindSafe` for `DropTracer`  
- `RefUnwindSafe` for `DropTracer`

### Changed

- `LogList` trait requires `len` method.
- `LogList` trait requires `Index` trait.
- `TraceVal::tracer` arguments
- Polish documentations.
- Update crate's dependencies.

## [0.6.1] - 2026-08-22

### Changed

- Improve performance.
- Polish documentations.

## [0.6.0] - 2026-06-24

### Added

- `DropTracer::dropped_count`
- `DropTracer::total_count`
- `DropTracer::trace_on`
- `TraceVal::tracer`

### Changed

- Polish documentations.
- Minor refactoring.

## [0.5.0] - 2026-06-20

### Fixed

- Double panic on drop of `TestVal` ([#2][issue2])

### Changed

- `DropTracer` supports multithreading ([#1][issue1])

## [0.4.0] - 2026-06-19

### Added

- Panic by double drops.

## [0.3.0] - 2026-06-18

### Changed

- Totally changed.

## [0.2.1] - 2025-07-14

### Changed

- Polish documentation.

## [0.2.0] - 2025-06-23

### Added

- Add `prelude` module (Although this crate is very small).

### Changed

- Rust edition is updated to 2024.
- Polish documentations.
- Minor refactoring.

## [0.1.1] - 2024-02-08

### Added

- Add implement `Debug` for `DropItem`.

## [0.1.0] - 2024-02-07

### Added

- First release.

[Unreleased]: https://github.com/nossie531/drop_tracer/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/nossie531/drop_tracer/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/nossie531/drop_tracer/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/nossie531/drop_tracer/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/nossie531/drop_tracer/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/nossie531/drop_tracer/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/nossie531/drop_tracer/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/nossie531/drop_tracer/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/nossie531/drop_tracer/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/nossie531/drop_tracer/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/nossie531/drop_tracer/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/nossie531/drop_tracer/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/nossie531/drop_tracer/releases/tag/v0.1.0
[issue1]: https://github.com/nossie531/drop_tracer/issues/1
[issue2]: https://github.com/nossie531/drop_tracer/issues/2