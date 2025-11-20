# better_unwrap

A Rust crate providing clearer alternatives to `unwrap()` and related methods.

## Why?

The standard library's `unwrap()` method name doesn't clearly communicate that it can panic. This crate provides methods with more explicit names:

- `or_panic()` instead of `unwrap()`
- `panic_or()` instead of `unwrap_or()`
- `panic_or_else()` instead of `unwrap_or_else()`
- `panic_or_default()` instead of `unwrap_or_default()`
- `panic_with()` instead of `expect()`

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
better_unwrap = "0.1.0"
```

Then use it:

```rust
use better_unwrap::prelude::*;

// Instead of .unwrap()
let value: u32 = result.or_panic();

// Instead of .unwrap_or(42)
let value: u32 = result.panic_or(42);

// Instead of .unwrap_or_else(|| compute())
let value: u32 = result.panic_or_else(|| compute());

// Instead of .unwrap_or_default()
let value: String = result.panic_or_default();

// Instead of .expect("message")
let value: u32 = result.panic_with("Custom error message");
```

All methods work with both `Result<T, E>` and `Option<T>`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

