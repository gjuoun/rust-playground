// Here are some common ways of error handling in Rust:

// 1. Panicking
// Panicking is used when a program encounters an unrecoverable error.
// For example, if you try to access an out - of - bounds index in a vector.
fn panic_example() {
    let v = vec![1, 2, 3];
    // This will panic
    // let x = v[10];
}

// 2. Result type
// The Result type is used for recoverable errors. It is an enum with two variants: Ok and Err.
// Here is an example of a function that returns a Result.
use std::num::ParseIntError;

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

// 3. Option type
// The Option type is used when a value may or may not exist. It is an enum with two variants: Some and None.
// Here is an example of a function that returns an Option.
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

// 4. The? operator
// The? operator is a convenient way to propagate errors in functions that return a Result or an Option.
fn parse_and_add(s1: &str, s2: &str) -> Result<i32, ParseIntError> {
    let num1 = s1.parse::<i32>()?;
    let num2 = s2.parse::<i32>()?;
    Ok(num1 + num2)
}

fn main() {
    panic_example();
    result_example();
    option_example();
    match parse_and_add("10", "20") {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error adding numbers: {}", e),
    }
}
