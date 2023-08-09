use std::net::SocketAddr;

use crate::bootstrap::bootstrap;

mod bootstrap;
mod error;
mod handlers;
mod models;
mod routes;
mod avored_state;
mod repositories;
mod providers;
mod requests;
mod middleware;
mod responses;

const PER_PAGE: i64 = 10;

#[tokio::main]
async fn main() {
    let router = bootstrap().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server Started: http://localhost:8080");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
