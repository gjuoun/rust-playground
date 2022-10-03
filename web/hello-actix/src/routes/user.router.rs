use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Scope};

pub fn get_router() -> Scope {
    web::scope("/users")
        .service(hello_user)
}


#[get("/")]
pub async fn hello_user() -> impl Responder {
    HttpResponse::Ok().body("Hello users!")
}



