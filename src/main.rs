use std::collections::HashMap;

use crate::{app::APIResponse, rules::CreateUser};

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use validator::Validate;

mod app;
mod config;
mod database;
mod model;
mod repository;
mod rules;

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
    let router = Router::new()
        .route("/health", get(health))
        .route("/users", post(create_user));

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
        errors: None,
    })
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> Json<APIResponse<&'static str>> {
    // validator user create
    println!("Payload: {:?}", payload);
    if let Err(errors) = payload.validate() {
        let mut error_messages: HashMap<String, Vec<String>> = HashMap::new();
        for (field, field_errors) in errors.field_errors() {
            let messages: Vec<String> = field_errors
                .iter()
                .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                .collect();
            error_messages.insert(field.to_string(), messages);
        }
        return Json(APIResponse {
            code: StatusCode::BAD_REQUEST.as_u16(),
            status: false,
            message: "Validation Error".to_string(),
            data: None,
            errors: Some(error_messages),
        });
    }
    Json(APIResponse {
        code: StatusCode::OK.as_u16(),
        status: true,
        message: "User created".to_string(),
        data: Some("User created"),
        errors: None,
    })
}
