use axum::{middleware, routing::{ delete, get, post, put }, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::cors::{ Any, CorsLayer };

mod controller;
mod model;
mod utils;

use controller::{auth_controller, category_controller, file_controller, http_controller, user_controller};
use utils::route_guard::auth_guard;

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

    print!("Listening on {} ", listener.local_addr().unwrap());

    let cors = CorsLayer::new().allow_origin(Any);

    let app_router = Router::new()
    .route("/", get(|| async { "Hello World" }))
    /* Category Route */
    .route("/api/category/search-paginate", post(category_controller::search_paginate))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/api/category", get(category_controller::find_many))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/api/category", post(category_controller::create))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/api/category/{id}", put(category_controller::update))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/api/category/{id}", delete(category_controller::delete))
    .route_layer(middleware::from_fn(auth_guard))

    /* User Route */
    .route("/api/user/search-paginate", post(user_controller::search_paginate))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/api/user", post(user_controller::create))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/api/user/{id}", put(user_controller::update))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/api/user/{id}", delete(user_controller::delete))
    .route_layer(middleware::from_fn(auth_guard))

    /* Auth Route */
    .route("/api/auth/login", post(auth_controller::login))
    .route("/api/auth/authenticated", post(auth_controller::authenticated))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/api/auth/change-password", post(auth_controller::change_password))
    .route_layer(middleware::from_fn(auth_guard))
    
    /* Http Example Route */
    .route("/api/http", get(http_controller::get_http_example))
    .route("/api/http", post(http_controller::post_http_example))
    /* Upload User File Route */
    .route("/api/files/user", post(file_controller::upload_user_image))
    .route("/api/files/user/image/{filename}", get(file_controller::get_user_image))
    .route("/api/files/user/delete/{filename}", delete(file_controller::delete_user_image))
    .layer(cors)
    .with_state(db_pool);

    axum::serve(listener, app_router).await.expect("Error while serving the server.");
}
