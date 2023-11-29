mod handler;

use axum::routing::{get, post, Router};
use handler::{hello_world, index};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/clicked", post(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
