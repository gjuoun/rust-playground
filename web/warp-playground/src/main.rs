use warp::Filter;

#[tokio::main]
async fn main() {

    let hello = warp::path!("hello" / String).map(|name| {
        format!("Hello, {}!", name)
    });

    let sum = 
    warp::path!("sum" / u32 / u32).map(|a, b| {
        format!("{} + {} = {}", a, b, a + b)
    });

    let times = 
    warp::path!(u16 / "times" / u16).map(|a, b| {
        format!("{} times {} = {}", a, b, a * b)
    });

    let math = warp::path("math")
    .and(sum.or(times));

    let routes = hello.or(math);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}