use axum::{routing::{ delete, get, post, put }, Router};
use controller::{category_controller, user_controller};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod controller {
    pub mod category_controller;
    pub mod user_controller;
}

#[tokio::main]
async fn main() {
    
    dotenvy::dotenv().expect("Failed to load .env file.");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("localhost:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("Database Url from .env file not found.");

    let db_pool = PgPoolOptions::new()
    .max_connections(16)
    .connect(&database_url)
    .await
    .expect("Failed to connect to the Database.");

    let listener = TcpListener::bind(server_address)
    .await.expect("Couldn't create TCP Listener.");

    print!("Listening on {}", listener.local_addr().unwrap());

    let app_router = Router::new()
    .route("/", get(|| async { "Hello World" }))
    /* Category Route */
    .route("/api/category", get(category_controller::find_many))
    .route("/api/category", post(category_controller::create))
    .route("/api/category/{id}", put(category_controller::update))
    .route("/api/category/{id}", delete(category_controller::delete))
    /* User Route */
    .route("/api/user/search-paginate", post(user_controller::search_paginate))
    .route("/api/user", post(user_controller::create))
    .route("/api/user/{id}", put(user_controller::update))
    .route("/api/user/{id}", delete(user_controller::delete))
    .with_state(db_pool);

    axum::serve(listener, app_router).await.expect("Error while serving the server.");
}
