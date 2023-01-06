#![deny(warnings)]
// use futures::io::Empty;
// use futures::TryStreamExt as _;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // let method = req.method();
    // let path = req.uri().path();
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") =>  {
            *response.body_mut() = "Something is good".into();
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }

        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }
    


    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
