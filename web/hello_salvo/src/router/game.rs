use salvo::{handler, prelude::Extractible, writer::Json, Request, Response, Router};
use serde::{Deserialize, Serialize};

pub fn game_router() -> Router {
    Router::with_path("/game/").push(Router::with_path("<id>").get(game).post(game_details))
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

#[derive(Serialize, Deserialize, Extractible, Debug, Clone, Copy)]
/// Get the data field value from the body by default.
#[extract(default_source(from = "body", format = "json"))]
struct GoodMan<'a> {
    /// The id number is obtained from the request path parameter, and the data is automatically parsed as i64 type.
    #[extract(source(from = "param"))]
    id: i64,
    /// Reference types can be used to avoid memory copying.
    username: &'a str,
}

// #[handler]
// async fn game_details(req: &mut Request) -> String {
//     let good_man: GoodMan<'_> = req.extract().await.unwrap();

//     format!(
//         "Good man is id: {}, name: {}",
//         good_man.id, good_man.username
//     )
// }

#[handler]
async fn game_details<'a>(good_man: GoodMan<'a>, res: &mut Response) {
    res.render(Json(good_man))
}
