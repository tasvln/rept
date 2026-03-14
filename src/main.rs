use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod states;
mod utils;

use crate::states::server::ServerState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let gemini_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let state = ServerState {
        db,
        jwt_secret,
        gemini_key,
    };

    let app = routes::create_router(state.clone()).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
