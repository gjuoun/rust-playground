use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("Searching for {}", filename);


    let contents = fs::read_to_string("./src/poem.txt").expect("Something wrong reading the file");
    println!("with text: \n {}", contents)

}
