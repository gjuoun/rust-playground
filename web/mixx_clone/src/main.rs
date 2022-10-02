use std::env;
use dotenv::dotenv;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Serialize, Deserialize)]
struct Student {
    name: String,
    age: u8,
}

#[get("/student")]
async fn get_one_student() -> impl Responder {
    let student = Student {
        name: "John".to_string(),
        age: 18,
    };
    HttpResponse::Ok().json(student)
}

#[post("/student")]
async fn post_one_student(student: web::Json<Student>) -> impl Responder {
    println!("name: {}, age: {}", student.name, student.age);
    HttpResponse::Ok().json(student)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = match env::var("PORT"){
        Ok(val) => val.parse::<u16>().unwrap_or(8080),
        Err(_e) => 8080,
    };


    let server = match HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_one_student)
            .service(post_one_student)
    })
    .bind(("127.0.0.1", port)) {
        Ok(server) => server,
        Err(e) => {
            println!("Server Error: {}", e);
            return Ok(());
        }
    };

    println!("Listening on port {}", port);
    server.run().await
}


