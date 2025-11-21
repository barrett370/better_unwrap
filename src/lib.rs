pub mod option;
pub mod result;

pub use option::BUOption;
pub use result::BUResult;

/// A prelude for conveniently importing the traits.
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
/// // panic_or_else can use the error value for Result
/// let err_result2: Result<u32, &str> = Err("failed");
/// let computed = err_result2.panic_or_else(|err| {
///     eprintln!("Error: {}", err);
///     100
/// });
///
/// // Custom panic message
/// let none_option: Option<u32> = None;
/// // none_option.panic_with("Expected a value here");
///
/// // Unwrap error variant (Result only - panics if Ok)
/// let err_result3: Result<u32, &str> = Err("error");
/// let error = err_result3.or_panic_err();
///
/// let err_result4: Result<u32, &str> = Err("error");
/// let error2 = err_result4.panic_err_with("Expected an error");
/// ```
pub mod prelude {
    pub use crate::{BUOption, BUResult};
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
        assert_eq!(result.panic_or_else(|_| 200), 42);
    }

    #[test]
    fn test_panic_or_else_with_result_err() {
        let result: Result<u32, &str> = Err("error");
        assert_eq!(result.panic_or_else(|err| {
            assert_eq!(err, "error");
            200
        }), 200);
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

    #[test]
    fn test_or_panic_err_with_err() {
        let result: Result<u32, &str> = Err("error message");
        assert_eq!(result.or_panic_err(), "error message");
    }

    #[test]
    fn test_or_panic_err_with_string_error() {
        let result: Result<u32, String> = Err("error".to_string());
        assert_eq!(result.or_panic_err(), "error".to_string());
    }

    #[test]
    #[should_panic(expected = "called `or_panic_err()` on an `Ok` value")]
    fn test_or_panic_err_panics_on_ok() {
        let result: Result<u32, &str> = Ok(42);
        let _ = result.or_panic_err();
    }

    #[test]
    fn test_panic_err_with_with_err() {
        let result: Result<u32, &str> = Err("error message");
        assert_eq!(result.panic_err_with("should not panic"), "error message");
    }

    #[test]
    #[should_panic(expected = "Custom error message")]
    fn test_panic_err_with_panics_on_ok() {
        let result: Result<u32, &str> = Ok(42);
        let _ = result.panic_err_with("Custom error message");
    }
}
