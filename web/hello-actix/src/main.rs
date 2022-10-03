use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
mod routes;
use routes::user;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    tls_builder
        .set_private_key_file("./tls/key.pem", SslFiletype::PEM)
        .unwrap();
        tls_builder.set_certificate_chain_file("./tls/cert.pem").unwrap();


    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service( user::get_router())
        
    })
    // .bind_openssl(("127.0.0.1", 8080), tls_builder)?;
    .bind(("127.0.0.1", 8080))?;
    

    println!("Listening on http://{}", server.addrs()[0]);
    server.run().await
}