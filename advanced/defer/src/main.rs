struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}

use std::ops::Deref;

// implement Deref for MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

use std::ops::DerefMut;

// implement DerefMut for MyBox,it needs Deref first
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn main() {
    let mut s = MyBox::new(String::from("hello, "));
    display(&mut s)
}

fn display(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}
