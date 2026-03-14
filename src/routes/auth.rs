use crate::{
    handlers::auth::{login, register},
    states::server::ServerState,
};
use axum::{Router, routing::post};

pub fn auth_router() -> Router<ServerState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
