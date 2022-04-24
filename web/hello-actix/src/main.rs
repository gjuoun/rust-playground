use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

async fn index() -> impl Responder {
    MyObj { name: "user" }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| actix_web::App::new().route("/", actix_web::web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
