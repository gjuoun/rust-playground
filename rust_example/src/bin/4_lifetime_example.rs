// lifetime_example.rs

// A struct with a reference, which needs a lifetime
struct Book<'a> {
    title: &'a str,
}

// A function that returns the longer of two string slices
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// A function that attempts to return a reference to a local variable
fn invalid_reference<'a>() -> &'a str {
    let local_string = String::from("I am local");
    &local_string
}

// A function that demonstrates a failed borrow due to lifetime issues
fn failed_borrow<'a>() {
    // `_x` is a local variable with a lifetime limited to this function.
    // It is stored on the stack and will be deallocated when the function exits.
    let _x = 12;

    // ERROR: `_x` does not live long enough
    // `_y` is a reference to `_x` with a specified lifetime `'a`.
    // The problem here is that `_x` will be dropped at the end of the function,
    // but `_y` is declared to have a lifetime `'a`, which suggests it should
    // live longer than the function scope.
    let _y: &'a i32 = &_x;
    // The reference `_y` cannot outlive `_x`, as `_x` will be out of scope
    // and deallocated once the function exits. Therefore, Rust's borrow checker
    // prevents this, resulting in a compile-time error.
}

fn main() {
    let string1 = String::from("Hello, world!");
    let string2 = "Rust is great!";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    let book = Book {
        title: "Rust Programming",
    };
    println!("The book title is: {}", book.title);

    // Uncommenting the following line will cause a compile-time error
    // let invalid = invalid_reference();
    // println!("Invalid reference: {}", invalid);

    // Uncommenting the following line will cause a compile-time error
    // failed_borrow();
}
