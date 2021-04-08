fn main() {
    let mut x = 5;
    println!("The value is {}", x);

    x= 100_000_000;
    println!("The value is {}", x);

    const y: u32 = 100_000;
    println!("The value is {}", y);

    let mut spaces = "      ";
    println!("The spece's length is {}", spaces.len());

    let mut spaces = "    ";
    spaces = spaces.len();
}
