fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let mut guess = String::from("Something");
    println!("{}",guess);

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("really long string");
    "halllo"
}

