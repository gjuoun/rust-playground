use futures::executor::block_on;

async fn do_something() {
    println!("go go go !");
}

fn main() {
    let future = do_something();
    block_on(future);
}