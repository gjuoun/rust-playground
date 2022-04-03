fn main() {
    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable
    // println!("{}", apples);
    // println!("{}", bananas);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // reuse same name, but with different type
    println!("{}", spaces)
}
