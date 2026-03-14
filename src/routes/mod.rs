pub mod auth;
pub mod receipt;

use crate::{middleware::auth::auth_middleware, states::server::ServerState};
use axum::{Router, middleware};

pub fn create_router(state: ServerState) -> Router<ServerState> {
    let protected =
        Router::new()
            .merge(receipt::receipts_router())
            .layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ));

    Router::new().merge(auth::auth_router()).merge(protected)
}
