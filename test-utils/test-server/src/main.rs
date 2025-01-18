use axum::{
    routing::get,
    Router,
};

use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/slow_answer", get(slow_answer));

    let listener = tokio::net::TcpListener::bind("localhost:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn slow_answer() -> &'static str {
    thread::sleep(Duration::from_secs(30));
    "Hello, World!"
}