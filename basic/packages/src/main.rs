use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
