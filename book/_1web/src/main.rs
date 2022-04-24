use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Listening on port 3000");

    server
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run()
        .expect("Can not start server");
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("n and m must be non-zero");
    }

    let response = format!(
        "The greatest common divisor of {} and {} is {}",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <html>
            <title>GCD Calculator</title>
            <body>
                <h1>GCD Calculator</h1>
                <form action="/gcd" method="post">
                    <input type="text" name="m"/>
                    <input type="text" name="n"/>
                    <button type="submit">Compute GCD</button>
                </form>
            </body>
        </html>
        "#,
    )
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
