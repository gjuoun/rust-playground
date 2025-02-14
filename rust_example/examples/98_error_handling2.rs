// create example to handle these errors:
// 1. panic
// 2. error
// 3. result
// 4. option
// 5. try
// 6, custom error

// region option_example
fn get_element_at_index(v: &[i32], index: usize) -> Option<i32> {
    if index < v.len() {
        Some(v[index])
    } else {
        None
    }
}

fn option_example() {
    let v = vec![1, 2, 3];
    match get_element_at_index(&v, 1) {
        Some(num) => println!("Element at index 1: {}", num),
        None => println!("Index out of bounds"),
    }
}
// endregion option_example

// region try_example
// 5. try
use std::num::ParseIntError;

fn parse_and_add(s1: &str, s2: &str) -> Result<i32, ParseIntError> {
    let num1 = s1.parse::<i32>()?;
    let num2 = s2.parse::<i32>()?;
    Ok(num1 + num2)
}

fn try_example() {
    let num_str1 = "123";
    let num_str2 = "456";
    match parse_and_add(num_str1, num_str2) {
        Ok(result) => println!("Sum: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
// endregion try_example

// region custom error
use std::error::Error;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
struct CustomError {
    details: String,
}

impl CustomError {
    fn new(msg: &str) -> CustomError {
        CustomError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CustomError {}

// Function that returns a custom error
fn might_fail() -> Result<(), CustomError> {
    Err(CustomError::new("This operation failed!"))
}

fn custom_error_example() {
    match might_fail() {
        Ok(_) => println!("Operation succeeded"),
        Err(e) => println!("Custom error: {}", e),
    }
}
// endregion custom error

// region panic_example

fn panic_example() {
    let v = vec![1, 2, 3];
    // This will panic
    // let _x = v[10];
}

// endregion panic_example

// region result_example
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

fn result_example() {
    let num_str = "123";
    match parse_number(num_str) {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error parsing number: {}", e),
    }
}

// endregion result_example

fn main() {
    println!("Panic example:");
    panic_example();

    println!("\nResult example:");
    result_example();

    println!("\nOption example:");
    option_example();

    println!("\nTry example:");
    try_example();

    println!("\nCustom error example:");
    custom_error_example();
}
