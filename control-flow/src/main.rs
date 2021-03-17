fn main() {
    let mut x = 10;

    let y = if x < 10 {5} else {20};

    println!("{}", y);

    let z = loop{
        println!("{}", x);
        x +=1;
        if x >= 30 {
            break x;
        }
    };

    println!("z is {}", z);
}
