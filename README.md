# better_unwrap

A Rust crate providing clearer alternatives to `unwrap()` and related methods.

## Why?

The standard library's `unwrap()` method name doesn't clearly communicate that it can panic. This crate provides methods with more explicit names:

- `or_panic()` instead of `unwrap()`
- `panic_or()` instead of `unwrap_or()`
- `panic_or_else()` instead of `unwrap_or_else()`
- `panic_or_default()` instead of `unwrap_or_default()`
- `panic_with()` instead of `expect()`
- `or_panic_err()` instead of `unwrap_err()` (Result only)
- `panic_err_with()` instead of `expect_err()` (Result only)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
better_unwrap = "1.0.0"
```

Then use it:

```rust
use better_unwrap::prelude::*;

// Instead of .unwrap()
let value: u32 = result.or_panic();

// Instead of .unwrap_or(42)
let value: u32 = result.panic_or(42);

// Instead of .unwrap_or_else(|err| compute(err))
// For Result: closure receives the error value
let value: u32 = result.panic_or_else(|err| compute(err));
// For Option: closure takes no parameters
let value: u32 = option.panic_or_else(|| compute());

// Instead of .unwrap_or_default()
let value: String = result.panic_or_default();

// Instead of .expect("message")
let value: u32 = result.panic_with("Custom error message");

// Instead of .unwrap_err() (Result only - panics if Ok)
let error: &str = result.or_panic_err();

// Instead of .expect_err("message") (Result only - panics if Ok)
let error: &str = result.panic_err_with("Expected an error");
```

Most methods work with both `Result<T, E>` and `Option<T>`. The `*_err()` methods (`or_panic_err()`, `panic_err_with()`) are only available for `Result<T, E>` since `Option<T>` doesn't have an error variant.

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

