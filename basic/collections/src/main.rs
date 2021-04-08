use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();

    let key = "John";

    scores.insert(key, 100);
    scores.insert("Guo", 100);

    println!(
        "score: {} - [{}]",
        key,
        match scores.get("John") {
            Some(v) => v,
            None => &1222002,
        }
    );

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
