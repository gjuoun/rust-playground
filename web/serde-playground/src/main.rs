use serde::{ Serialize, Deserialize};
use serde_json::{ json};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}


fn main() {
    let person = Person {
        name: "John Doe".to_owned(),
        age: 43,
        phones: vec!["+44 1234567".to_owned(), "+44 2345678".to_owned()],
    };

    println!("{}", serde_json::to_string(&person).unwrap());

}
