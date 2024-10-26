mod db;
mod api;

use axum::{Router, routing::get, routing::post};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .nest("/", api::router())
        .with_state(pool.clone())
        .layer(CorsLayer::very_permissive());

    let address = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Starting server on http://{address}");
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}
