#[macro_use]
extern crate rocket;
use rocket::form::{Form, FromForm};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
