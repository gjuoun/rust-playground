fn main() {
    // Print using println!
    println!("Hello, world!");

    // Print with formatting
    println!("I'm a Rustacean!");

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting
    println!("Base 10:               {}", 69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);

    // Right-align text with specified width
    println!("{number:>5}", number=1);
    println!("{number:>5}", number=10);
    println!("{number:>5}", number=100);
}
