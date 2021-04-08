fn main() {
    let mut x = 10;

    let y = if x < 10 { 5 } else { 20 };

    println!("{}", y);

    // for loop
    let z = loop {
        println!("{}", x);
        x += 1;
        if x >= 30 {
            break x;
        }
    };

    println!("z is {}", z);

    // while loop
    let mut number = 3;
    while number != 0{
        println!("{}", number);
        number -= 1;
    }

    println!("LIFTOFF");

    let a = [1,2,3,4,5,6,7,8, 9, 10];
    for element in a.iter() {
        println!("the number is {}", element);
    }


    let range = 1..10;
    
    for num in range.rev() {
        println!("iter - {}", num);
    }
}
