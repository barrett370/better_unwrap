use better_unwrap::prelude::*;

fn main() {
    // Example 1: Basic or_panic() with Result::Ok
    let result: Result<u32, &str> = Ok(42);
    let value = result.or_panic();
    println!("Result success: {}", value);

    // Example 2: Basic or_panic() with Option::Some
    let option: Option<String> = Some("hello".to_string());
    let value2 = option.or_panic();
    println!("Option success: {}", value2);

    // Example 3: panic_or() - provides default instead of panicking
    let err_result: Result<u32, &str> = Err("something went wrong");
    let defaulted = err_result.panic_or(0);
    println!("Used default value: {}", defaulted);

    let none_option: Option<u32> = None;
    let defaulted2 = none_option.panic_or(100);
    println!("Used default value: {}", defaulted2);

    // Example 4: panic_or_else() - compute default via closure
    let err_result2: Result<u32, &str> = Err("error");
    let computed = err_result2.panic_or_else(|| {
        println!("Computing default value...");
        2 * 21
    });
    println!("Computed value: {}", computed);

    // Example 5: panic_or_default() - uses Default trait
    let err_result3: Result<String, &str> = Err("error");
    let default_string = err_result3.panic_or_default();
    println!("Default string: '{}'", default_string);

    // Example 6: panic_with() - custom panic message
    let ok_result: Result<u32, &str> = Ok(42);
    let value3 = ok_result.panic_with("This should not panic");
    println!("Value with custom message: {}", value3);

    // Uncomment to see panic behavior:
    // let error_result: Result<u32, &str> = Err("something went wrong");
    // error_result.or_panic(); // This will panic

    // let none_option: Option<u32> = None;
    // none_option.panic_with("Expected a value here"); // This will panic with custom message
}

