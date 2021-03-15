use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;
use std::string::String;

fn main() {
  println!("Guess the number!");

  let secret_number = thread_rng().gen_range(1..101);
  println!("The secret_number is {}", secret_number);

  loop {
    println!("Please input your gueess");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("is wrong");

    println!("You guessed :{}", guess);

    let guess:u32 = guess.trim().parse().unwrap();
    
    match guess.cmp(&secret_number) {
      Ordering::Equal => {
        println!("You win!");
        break;
      }
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big"),
    }
  }
}
