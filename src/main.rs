mod endpoints;

use axum::{
    routing::{get, post},
    Router,
};
use endpoints::{eval, home_handler};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/eval", post(eval));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
