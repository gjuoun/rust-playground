fn main() {
    // Rust code use `snake case` as the conventional style for cuntion and variable names
    // snake caasdfe all letters are lowercase and underscores separate workds.

    
    println!("{}",another_function(565));

    let x = 5;

    let y = {
        let x = 3;
        x+1
    };

    println!("The value of y is {}", y);
}


fn another_function(x: i32) -> i32 {
    x
}

