use axum::response::Html;
use axum::{Router, ServiceExt};
use axum::routing::get;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routers_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello, world!") })
    );

    // region:    ---Start Server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routers_hello.into_make_service())
        .await
        .unwrap();
}
