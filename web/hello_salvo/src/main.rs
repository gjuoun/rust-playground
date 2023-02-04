mod router;
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    let root_router = Router::new()
        .push(router::health::health_router())
        .push(router::game::game_router());

    let addr = "127.0.0.1:7878";
    let acceptor = TcpListener::bind(&addr);
    println!("Listening on http://{}", addr);
    Server::new(acceptor).serve(root_router).await;
}
