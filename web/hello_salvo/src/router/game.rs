use salvo::{handler, Request, Router};
use serde::Deserialize;

pub fn game_router() -> Router {
    Router::with_path("/game/<id>").get(game).post(game_details)
}

#[handler]
async fn game(req: &mut Request) -> String {
    let id = req.param::<&str>("id").unwrap_or("Game doesn't exist");
    let query = req.query::<&str>("q").unwrap_or("No query");
    return format!("Game: {}, q= {}", id, query);
}

#[derive(Deserialize)]
struct GameDetails {
    id: String,
    title: String,
}

#[handler]
async fn game_details(req: &mut Request) -> String {
    let other_game = req.parse_json::<GameDetails>().await.unwrap();
    return format!("Game, {}", other_game.title);
}
