use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{states::server::ServerState, utils::jwt::verify_token};

pub async fn auth_middleware(
    State(state): State<ServerState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t.to_string(),
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "message": "Missing Bearer token" })),
            ));
        }
    };

    let claims = match verify_token(&token, &state.jwt_secret) {
        Some(c) => c,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "message": "Invalid Bearer token" })),
            ));
        }
    };

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
