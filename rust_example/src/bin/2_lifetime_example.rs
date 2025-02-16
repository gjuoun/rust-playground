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

fn main() {
    let string1 = String::from("Hello, world!");
    let string2 = "Rust is great!";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    let book = Book { title: "Rust Programming" };
    println!("The book title is: {}", book.title);
}
