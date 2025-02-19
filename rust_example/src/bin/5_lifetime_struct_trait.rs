#[derive(Debug)]
struct MultiBorrowed<'a, 'b> {
    first: &'a i32, 
    second: &'b i32,
}

trait Sum {
    fn sum(&self) -> i32;
}

impl<'a, 'b> Sum for MultiBorrowed<'a, 'b> {
    fn sum(&self) -> i32 {
        *self.first + *self.second
    }
}

fn main() {
    let num1 = 5;
    let num2 = 10;

    let multi = MultiBorrowed {
        first: &num1,
        second: &num2,
    };

    let result = multi.sum();
    println!("The sum of the borrowed numbers is: {}", result);
}
