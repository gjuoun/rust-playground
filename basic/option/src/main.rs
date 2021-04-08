enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            return 1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let x: i8 = 5;
    let y: Coin = Coin::Quarter(String::from("John Guo"));

    value_in_cents(y);
}
