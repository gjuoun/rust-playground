use std::fmt::Debug;

// Define a struct that holds a reference to a generic type T
// The type T must satisfy two conditions:
// 1. It must implement the Debug trait so we can print it.
// 2. All references inside T must outlive the lifetime 'a.
#[derive(Debug)]
struct Wrapper<'a, T: 'a + Debug> {
    value: &'a T,
}

// Function to print a value that implements Debug
fn print_value<T>(t: T)
where
    T: Debug,
{
    println!("Printing value: {:?}", t);
}

// Function to print a reference to a value.
// The type T must implement Debug and all references in T must outlive 'a.
// Also, the lifetime 'a must outlive the function call.
fn print_ref_value<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("Printing reference to value: {:?}", t);
}

fn main() {
    let num = 42;
    let wrapped_num = Wrapper { value: &num };

    // Call the function to print the wrapped value
    print_value(wrapped_num);

    // Call the function to print a reference to the wrapped value
    print_ref_value(&wrapped_num);
}
