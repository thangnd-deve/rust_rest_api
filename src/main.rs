use crate::app::APIResponse;
use axum::{Json, Router, http::StatusCode, routing::get};

mod app;
mod config;
mod database;
mod model;
mod repository;
mod validator;

pub struct AppState {
    pub db: database::Database,
}

#[tokio::main]
async fn main() {
    config::load_config();
    init().await;
    run().await;
}

pub async fn init() -> AppState {
    config::load_config();
    let _db = database::Database::new().await;

    AppState { db: _db }
}

pub async fn run() {
    let router = Router::new().route("/health", get(health));

    // load config server start
    let host = config::get_config_by_key("SERVER_HOST", Some("127.0.0.1"));
    let port = config::get_config_by_key("SERVER_PORT", Some("8080"));

    let listner = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    println!("Server running at http://{}:{}", host, port);
    axum::serve(listner, router).await.unwrap();
}

pub async fn health() -> Json<APIResponse<&'static str>> {
    Json(APIResponse {
        code: StatusCode::OK.as_u16(),
        status: true,
        message: "health".to_string(),
        data: Some("health"),
    })
}
