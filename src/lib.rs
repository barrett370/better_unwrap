use std::fmt::Debug;
use std::default::Default;

/// Trait that provides methods as alternatives to `unwrap()` and related methods
///
/// This trait is implemented for both `Result<T, E>` and `Option<T>`,
/// allowing you to use clearer method names like `.or_panic()` instead of `.unwrap()`.
pub trait BetterUnwrap<T, E> {
    /// Unwraps a result or option, yielding the content of an `Ok` or `Some`.
    ///
    /// # Panics
    ///
    /// - For `Result`: Panics if the value is an `Err`, with a panic message
    ///   including the error value formatted using `Debug`.
    /// - For `Option`: Panics if the value is `None`.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use better_unwrap::BetterUnwrap;
    ///
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.or_panic(); // panics with `"emergency failure"`
    /// ```
    ///
    /// ```should_panic
    /// use better_unwrap::BetterUnwrap;
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
    /// use better_unwrap::BetterUnwrap;
    ///
    /// let x: Result<u32, &str> = Err("error");
    /// assert_eq!(x.panic_or(42), 42);
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
    /// use better_unwrap::BetterUnwrap;
    ///
    /// let x: Result<u32, &str> = Err("error");
    /// assert_eq!(x.panic_or_else(|| 2 * 21), 42);
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
    /// use better_unwrap::BetterUnwrap;
    ///
    /// let x: Result<u32, &str> = Err("error");
    /// assert_eq!(x.panic_or_default(), 0);
    ///
    /// let y: Option<String> = None;
    /// assert_eq!(y.panic_or_default(), String::new());
    /// ```
    fn panic_or_default(self) -> T
    where
        T: Default;

    /// Unwraps a result or option, yielding the content of an `Ok` or `Some`.
    ///
    /// Equivalent to `expect()`, but with a clearer name.
    ///
    /// # Panics
    ///
    /// Panics if the value is an `Err` or `None`, with a panic message including the
    /// provided message.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use better_unwrap::BetterUnwrap;
    ///
    /// let x: Result<u32, &str> = Err("emergency failure");
    /// x.panic_with("Testing error handling"); // panics with `"Testing error handling"`
    /// ```
    ///
    /// ```should_panic
    /// use better_unwrap::BetterUnwrap;
    ///
    /// let x: Option<u32> = None;
    /// x.panic_with("Expected a value"); // panics with `"Expected a value"`
    /// ```
    fn panic_with(self, msg: &str) -> T;
}

impl<T, E: Debug> BetterUnwrap<T, E> for Result<T, E> {
    fn or_panic(self) -> T {
        match self {
            Ok(value) => value,
            Err(error) => panic!("called `or_panic()` on an `Err` value: {:?}", error),
        }
    }

    fn panic_or(self, default: T) -> T {
        self.unwrap_or(default)
    }

    fn panic_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.unwrap_or_else(|_| f())
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

impl<T> BetterUnwrap<T, ()> for Option<T> {
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

/// A prelude for conveniently importing the trait.
///
/// # Example
///
/// ```
/// use better_unwrap::prelude::*;
///
/// let result: Result<u32, &str> = Ok(42);
/// let value = result.or_panic();
///
/// let option: Option<String> = Some("hello".to_string());
/// let value2 = option.or_panic();
///
/// // Use default value instead of panicking
/// let err_result: Result<u32, &str> = Err("error");
/// let defaulted = err_result.panic_or(0);
///
/// // Custom panic message
/// let none_option: Option<u32> = None;
/// // none_option.panic_with("Expected a value here");
/// ```
pub mod prelude {
    pub use crate::BetterUnwrap;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or_panic_with_ok() {
        let result: Result<u32, &str> = Ok(42);
        assert_eq!(result.or_panic(), 42);
    }

    #[test]
    fn test_or_panic_with_string_error() {
        let result: Result<u32, String> = Ok(100);
        assert_eq!(result.or_panic(), 100);
    }

    #[test]
    #[should_panic(expected = "called `or_panic()` on an `Err` value")]
    fn test_or_panic_panics_on_err() {
        let result: Result<u32, &str> = Err("test error");
        let _ = result.or_panic();
    }

    #[test]
    fn test_or_panic_with_some() {
        let option: Option<u32> = Some(42);
        assert_eq!(option.or_panic(), 42);
    }

    #[test]
    fn test_or_panic_with_string_option() {
        let option: Option<String> = Some("hello".to_string());
        assert_eq!(option.or_panic(), "hello");
    }

    #[test]
    #[should_panic(expected = "called `or_panic()` on a `None` value")]
    fn test_or_panic_panics_on_none() {
        let option: Option<u32> = None;
        let _ = option.or_panic();
    }

    #[test]
    fn test_panic_or_with_result_ok() {
        let result: Result<u32, &str> = Ok(42);
        assert_eq!(result.panic_or(100), 42);
    }

    #[test]
    fn test_panic_or_with_result_err() {
        let result: Result<u32, &str> = Err("error");
        assert_eq!(result.panic_or(100), 100);
    }

    #[test]
    fn test_panic_or_with_option_some() {
        let option: Option<u32> = Some(42);
        assert_eq!(option.panic_or(100), 42);
    }

    #[test]
    fn test_panic_or_with_option_none() {
        let option: Option<u32> = None;
        assert_eq!(option.panic_or(100), 100);
    }

    #[test]
    fn test_panic_or_else_with_result_ok() {
        let result: Result<u32, &str> = Ok(42);
        assert_eq!(result.panic_or_else(|| 200), 42);
    }

    #[test]
    fn test_panic_or_else_with_result_err() {
        let result: Result<u32, &str> = Err("error");
        assert_eq!(result.panic_or_else(|| 200), 200);
    }

    #[test]
    fn test_panic_or_else_with_option_some() {
        let option: Option<u32> = Some(42);
        assert_eq!(option.panic_or_else(|| 200), 42);
    }

    #[test]
    fn test_panic_or_else_with_option_none() {
        let option: Option<u32> = None;
        assert_eq!(option.panic_or_else(|| 200), 200);
    }

    #[test]
    fn test_panic_or_default_with_result_ok() {
        let result: Result<u32, &str> = Ok(42);
        assert_eq!(result.panic_or_default(), 42);
    }

    #[test]
    fn test_panic_or_default_with_result_err() {
        let result: Result<u32, &str> = Err("error");
        assert_eq!(result.panic_or_default(), 0);
    }

    #[test]
    fn test_panic_or_default_with_option_some() {
        let option: Option<String> = Some("hello".to_string());
        assert_eq!(option.panic_or_default(), "hello".to_string());
    }

    #[test]
    fn test_panic_or_default_with_option_none() {
        let option: Option<String> = None;
        assert_eq!(option.panic_or_default(), String::new());
    }

    #[test]
    fn test_panic_with_result_ok() {
        let result: Result<u32, &str> = Ok(42);
        assert_eq!(result.panic_with("should not panic"), 42);
    }

    #[test]
    #[should_panic(expected = "Custom error message")]
    fn test_panic_with_result_err() {
        let result: Result<u32, &str> = Err("error");
        let _ = result.panic_with("Custom error message");
    }

    #[test]
    fn test_panic_with_option_some() {
        let option: Option<u32> = Some(42);
        assert_eq!(option.panic_with("should not panic"), 42);
    }

    #[test]
    #[should_panic(expected = "Custom error message")]
    fn test_panic_with_option_none() {
        let option: Option<u32> = None;
        let _ = option.panic_with("Custom error message");
    }
}
