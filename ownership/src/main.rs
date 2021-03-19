fn main() {

    let str = String::from("he llo world"); 

    let size = first_word(&str);

    println!("{}", size);
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}