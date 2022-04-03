fn main() {
    // Rust code use `snake case` as the conventional style for funtion and variable names
    // snake caasdfe all letters are lowercase and underscores separate workds.

    println!("{}", another_function(565));

    let _x = 5;

    let y = {
        let x = 3;
        x + 3
    };

    println!("The value of y is {}", y);

    let z = plus_one(5);
    println!(" z is {}", z);
}

fn another_function(x: i32) -> i32 {
    x
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
