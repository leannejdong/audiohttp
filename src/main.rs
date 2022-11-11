//use core::convert::Infallible;
use warp::Filter;
// struct Greet{
//     message: String,
// }
// async fn index(item: Greet)->Result<impl warp::Reply, Infallible>{
//     Ok(warp::reply::json(&hello.message))
// }

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello"/String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
