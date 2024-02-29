use std::env::var;
use axum::{Router};
use axum::http::HeaderValue;
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir};

mod constants {
    pub const DIR: &str = "DIR";
    pub const ALLOWED_ORIGINS : &str = "ALLOWED_ORIGINS";
    pub const DIR_NOT_FOUND : &str = "Variable DIR is not found";
    pub const ALLOWED_ORIGINS_NOT_FOUND : &str = "Variable ALLOWED_ORIGINS is not found";
    pub const ADDR: &str = "127.0.0.1:8080";
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let dir = var(constants::DIR).expect(constants::DIR_NOT_FOUND);

    let allowed_origins = var(constants::ALLOWED_ORIGINS)
        .map(|origins|
            origins.split(",")
                .into_iter()
                .map(|origin| origin.parse::<HeaderValue>().unwrap())
                .collect::<Vec<HeaderValue>>()
        ).expect(constants::ALLOWED_ORIGINS_NOT_FOUND);

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins);

    let app = Router::new()
        .nest_service("/", ServeDir::new(dir))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(constants::ADDR)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap()
}
