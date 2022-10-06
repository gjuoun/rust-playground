use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        
        let rc = Rc::new("hello");
        println!("{}", rc);
        // `rc` is used after `.await`. It must be persisted to
        // the task's state.
        yield_now().await;

    });
}