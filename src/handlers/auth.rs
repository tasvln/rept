use axum::{Json, extract::State, http::StatusCode};
use uuid::Uuid;

use crate::{
    models::user::User,
    states::server::ServerState,
    utils::{
        jwt::create_token,
        password::{hash_password, verify_password},
    },
};

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn register(
    State(state): State<ServerState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    if !validate_password(&payload.password) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message:
                    "Password must be 8+ chars, include uppercase, number and special character"
                        .to_string(),
            }),
        ));
    }

    let exists = sqlx::query("SELECT id FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    if exists.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                message: "Username already taken".to_string(),
            }),
        ));
    }

    let id = Uuid::new_v4();
    let password_hash = hash_password(&payload.password);

    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(&payload.username)
        .bind(&password_hash)
        .execute(&state.db)
        .await
        .unwrap();

    let token = create_token(&id.to_string(), &state.jwt_secret);
    Ok(Json(AuthResponse { token }))
}

pub async fn login(
    State(state): State<ServerState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash FROM users WHERE username = $1",
    )
    .bind(&payload.username)
    .fetch_optional(&state.db)
    .await
    .unwrap();

    match user {
        None => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                message: "Invalid credentials".to_string(),
            }),
        )),
        Some(u) => {
            if verify_password(&u.password_hash, &payload.password) {
                let token = create_token(&u.id.to_string(), &state.jwt_secret);
                Ok(Json(AuthResponse { token }))
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        message: "Invalid credentials".to_string(),
                    }),
                ))
            }
        }
    }
}

fn validate_password(password: &str) -> bool {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| "@$!%*?&".contains(c));
    let has_length = password.len() >= 8;

    has_upper && has_digit && has_special && has_length
}
