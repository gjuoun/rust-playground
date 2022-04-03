fn main() {
    let str = String::from("hello world");

    let slice = first_word2(&str);
    println!("{}", slice);
}

// function to find the first word in a string and return its size
fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // println!("{:?}", bytes);

    // let mut iterator = bytes.iter();
    // let first_letter = iterator.next();
    // println!("{:?}", first_letter.unwrap());

    // let mut enumerator = iterator.enumerate();
    // let value1 = enumerator.next();
    // println!("{:?}", value1.unwrap());

    for (i, &item) in bytes.iter().enumerate() {
        println!("{} - {}", i, item);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}