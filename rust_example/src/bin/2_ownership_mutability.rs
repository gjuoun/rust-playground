struct NumberBox {
    value: i32,
}

fn main() {
    // Example: Mutability with custom struct
    let immutable_box = Box::new(NumberBox { value: 10 });

    println!("immutable_box contains {}", immutable_box.value);

    // Move the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box.value);

    // Modify the contents of the box
    mutable_box.value = 20;

    println!("mutable_box now contains {}", mutable_box.value);
}
