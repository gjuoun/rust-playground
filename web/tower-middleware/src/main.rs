use std::time::Duration;

#[derive(Debug, Clone)]
struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S> Timeout<S> {
    pub fn new(inner: S, timeout: Duration) -> Self {
        Timeout { inner, timeout }
    }
}

fn main() {
    println!("Jun's playground");
}