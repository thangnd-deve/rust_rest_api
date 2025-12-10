use std::collections::HashMap;

use crate::{app::APIResponse, repository::UserRepository, rules::CreateUser};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use validator::Validate;

mod app;
mod config;
mod database;
mod helpers;
mod model;
mod repository;
mod rules;

#[derive(Clone)]
pub struct AppState {
    pub db: database::Database,
}

#[tokio::main]
async fn main() {
    config::load_config();
    let app = init().await;
    run(app).await;
}

pub async fn init() -> AppState {
    config::load_config();
    let _db = database::Database::new().await;

    AppState { db: _db }
}

pub async fn run(app: AppState) {
    let router = Router::new()
        .route("/health", get(health))
        .route("/users", post(create_user))
        .with_state(app);

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

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Json<APIResponse<String>> {
    // Validate input
    if let Err(errors) = payload.validate() {
        let mut error_messages: HashMap<String, Vec<String>> = HashMap::new();
        for (field, field_errors) in errors.field_errors() {
            let messages: Vec<String> = field_errors
                .iter()
                .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                .collect();
            error_messages.insert(field.to_string(), messages);
        }

        // Validation FAILED → Return error, DO NOT create user
        return Json(APIResponse {
            code: StatusCode::BAD_REQUEST.as_u16(),
            status: false,
            message: "Validation failed".to_string(),
            data: None,
            errors: Some(error_messages),
        });
    }

    // Validation OK → Safe to unwrap (required validation passed)
    let username = payload.username.unwrap();
    let email = payload.email.unwrap();
    let password = payload.password.unwrap();

    // Hash password
    let password_hash = helpers::hash_password(&password).unwrap();

    // Create user in database
    match UserRepository::create_user(&state.db, &username, &email, &password_hash).await {
        Ok(user_id) => Json(APIResponse {
            code: StatusCode::CREATED.as_u16(),
            status: true,
            message: "User created successfully".to_string(),
            data: Some(user_id.to_string()),
            errors: None,
        }),
        Err(e) => {
            println!("Database error: {}", e);
            Json(APIResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                status: false,
                message: "Failed to create user".to_string(),
                data: None,
                errors: None,
            })
        }
    }
}
