use std::fmt::Debug;
use std::default::Default;

/// Trait that provides methods as alternatives to `unwrap()` and related methods for `Result<T, E>`.
///
/// This trait allows you to use clearer method names like `.or_panic()` instead of `.unwrap()`.
pub trait BEResult<T, E> {
    /// Unwraps a result, yielding the content of an `Ok`.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Err`, with a panic message including the error value formatted using `Debug`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use better_unwrap::BEResult;
    ///
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.or_panic(); // panics with `"emergency failure"`
    /// ```
    fn or_panic(self) -> T;

    /// Returns the contained value or a provided default.
    ///
    /// Equivalent to `unwrap_or()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use better_unwrap::BEResult;
    ///
    /// let x: Result<u32, &str> = Err("error");
    /// assert_eq!(x.panic_or(42), 42);
    /// ```
    fn panic_or(self, default: T) -> T;

    /// Returns the contained value or computes it from a closure that receives the error.
    ///
    /// Equivalent to `unwrap_or_else()`. The closure receives the error value, allowing you to use it.
    ///
    /// # Examples
    ///
    /// ```
    /// use better_unwrap::BEResult;
    ///
    /// let x: Result<u32, &str> = Err("error");
    /// assert_eq!(x.panic_or_else(|err| {
    ///     eprintln!("Error: {}", err);
    ///     42
    /// }), 42);
    /// ```
    fn panic_or_else<F>(self, f: F) -> T
    where
        F: FnOnce(E) -> T;

    /// Returns the contained value or the default value for the type.
    ///
    /// Equivalent to `unwrap_or_default()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use better_unwrap::BEResult;
    ///
    /// let x: Result<u32, &str> = Err("error");
    /// assert_eq!(x.panic_or_default(), 0);
    /// ```
    fn panic_or_default(self) -> T
    where
        T: Default;

    /// Unwraps a result, yielding the content of an `Ok`.
    ///
    /// Equivalent to `expect()`, but with a clearer name.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Err`, with a panic message including the provided message.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use better_unwrap::BEResult;
    ///
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.panic_with("Testing error handling"); // panics with `"Testing error handling"`
    /// ```
    fn panic_with(self, msg: &str) -> T;

    /// Unwraps a result, yielding the content of an `Err`.
    ///
    /// Equivalent to `unwrap_err()`.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Ok`, with a panic message including the Ok value formatted using `Debug`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use better_unwrap::BEResult;
    ///
    /// let x: Result<u32, &str> = Ok(42);
    /// x.or_panic_err(); // panics with `"called or_panic_err() on an Ok value: 42"`
    /// ```
    ///
    /// ```
    /// use better_unwrap::BEResult;
    ///
    /// let x: Result<u32, &str> = Err("error message");
    /// let error = x.or_panic_err(); // returns "error message"
    /// assert_eq!(error, "error message");
    /// ```
    fn or_panic_err(self) -> E
    where
        T: Debug;

    /// Unwraps a result, yielding the content of an `Err`.
    ///
    /// Equivalent to `expect_err()`, but with a clearer name.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Ok`, with a panic message including the provided message.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use better_unwrap::BEResult;
    ///
    /// let x: Result<u32, &str> = Ok(42);
    /// x.panic_err_with("Expected an error"); // panics with `"Expected an error"`
    /// ```
    ///
    /// ```
    /// use better_unwrap::BEResult;
    ///
    /// let x: Result<u32, &str> = Err("error message");
    /// let error = x.panic_err_with("Should not panic"); // returns "error message"
    /// assert_eq!(error, "error message");
    /// ```
    fn panic_err_with(self, msg: &str) -> E;
}

impl<T, E: Debug> BEResult<T, E> for Result<T, E> {
    fn or_panic(self) -> T {
        match self {
            Ok(value) => value,
            Err(error) => panic!("called `or_panic()` on an `Err` value: {error:?}"),
        }
    }

    fn panic_or(self, default: T) -> T {
        self.unwrap_or(default)
    }

    fn panic_or_else<F>(self, f: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        self.unwrap_or_else(f)
    }

    fn panic_or_default(self) -> T
    where
        T: Default,
    {
        self.unwrap_or_default()
    }

    fn panic_with(self, msg: &str) -> T {
        self.expect(msg)
    }

    fn or_panic_err(self) -> E
    where
        T: Debug,
    {
        match self {
            Ok(value) => panic!("called `or_panic_err()` on an `Ok` value: {value:?}"),
            Err(error) => error,
        }
    }

    fn panic_err_with(self, msg: &str) -> E {
        match self {
            Ok(_) => panic!("{}", msg),
            Err(error) => error,
        }
    }
}

