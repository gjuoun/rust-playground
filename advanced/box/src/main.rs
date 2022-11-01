
#![allow(unused)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age}
    }

    fn display(self: &mut Person, age: u8) {
        
        let Person{name, age} = &self;
    }
}

 
fn main() {
    let s = gen_static_str();
    println!("{}", s);
}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push_str("gaga");

    Box::leak(s.into_boxed_str())
}
