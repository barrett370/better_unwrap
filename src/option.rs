use std::default::Default;

/// Trait that provides methods as alternatives to `unwrap()` and related methods for `Option<T>`.
///
/// This trait allows you to use clearer method names like `.or_panic()` instead of `.unwrap()`.
pub trait BEOption<T> {
    /// Unwraps an option, yielding the content of a `Some`.
    ///
    /// # Panics
    ///
    /// Panics if the value is `None`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use better_unwrap::BEOption;
    ///
    /// let x: Option<u32> = None;
    /// x.or_panic(); // panics with `"called or_panic() on a None value"`
    /// ```
    fn or_panic(self) -> T;

    /// Returns the contained value or a provided default.
    ///
    /// Equivalent to `unwrap_or()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use better_unwrap::BEOption;
    ///
    /// let y: Option<u32> = None;
    /// assert_eq!(y.panic_or(100), 100);
    /// ```
    fn panic_or(self, default: T) -> T;

    /// Returns the contained value or computes it from a closure.
    ///
    /// Equivalent to `unwrap_or_else()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use better_unwrap::BEOption;
    ///
    /// let y: Option<u32> = None;
    /// assert_eq!(y.panic_or_else(|| 100), 100);
    /// ```
    fn panic_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T;

    /// Returns the contained value or the default value for the type.
    ///
    /// Equivalent to `unwrap_or_default()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use better_unwrap::BEOption;
    ///
    /// let y: Option<String> = None;
    /// assert_eq!(y.panic_or_default(), String::new());
    /// ```
    fn panic_or_default(self) -> T
    where
        T: Default;

    /// Unwraps an option, yielding the content of a `Some`.
    ///
    /// Equivalent to `expect()`, but with a clearer name.
    ///
    /// # Panics
    ///
    /// Panics if the value is `None`, with a panic message including the provided message.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use better_unwrap::BEOption;
    ///
    /// let x: Option<u32> = None;
    /// x.panic_with("Expected a value"); // panics with `"Expected a value"`
    /// ```
    fn panic_with(self, msg: &str) -> T;
}

impl<T> BEOption<T> for Option<T> {
    fn or_panic(self) -> T {
        match self {
            Some(value) => value,
            None => panic!("called `or_panic()` on a `None` value"),
        }
    }

    fn panic_or(self, default: T) -> T {
        self.unwrap_or(default)
    }

    fn panic_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
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
}

