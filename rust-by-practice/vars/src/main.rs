#[derive(Debug)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

fn main() {
    let todo = Todo {
        id: 1,
        title: String::from("Learn Rust"),
        completed: false,
    };

    println!("Hello, world!");

    println!("todo = {:#?}", todo);

    println!("{number:>5}", number = 1);

    println!("{number:0>5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display
    println!("This struct `{:?}`    won't print...", Structure(3));

    let pi = 3.141592;

    println!("Pi is roughly {:.3}", pi);
}
