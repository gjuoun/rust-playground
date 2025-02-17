// 2_ownership.rs
// Demonstrating Rust's ownership system

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Takes ownership of the Person and returns it
fn celebrate_birthday(mut person: Person) -> Person {
    person.age += 1;
    println!(
        "Happy birthday {}! You are now {}!",
        person.name, person.age
    );
    person
}

// Takes ownership and returns it back
fn fix_ownership(person: Person) -> Person {
    person
}

fn update_username(person: &mut Person, new_name: &str) {
    let old_name = std::mem::replace(&mut person.name, new_name.to_string());
    println!("Updated username from '{}' to '{}'", old_name, person.name);
}

fn main() {
    // Example 1: Basic ownership transfer
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let bob = alice; // Ownership moves to bob
                     // println!("{:?}", alice); // This would error if uncommented

    // Example 2: Function parameter ownership
    let mut charlie = celebrate_birthday(bob);
    // println!("{:?}", bob); // bob is no longer valid here

    // Example 3: Clone for explicit duplication
    let david = Person {
        name: charlie.name.clone(),
        age: charlie.age,
    };

    charlie.name = String::from("Charles");
    println!("Original: {:?}", charlie);
    println!("Clone: {:?}", david);

    // Example 4: Fixing ownership errors
    let eve = fix_ownership(charlie);  // charlie is no longer valid here
    println!("Fixed ownership: {:?}", eve);
    
    let mut frank = eve;
    update_username(&mut frank, "Franklin");
    println!("Updated frank: {:?}", frank);
}
