use crate::{
    handlers::receipt::{get_receipt, get_receipts, upload_receipt},
    states::server::ServerState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn receipts_router() -> Router<ServerState> {
    Router::new()
        .route("/receipts", post(upload_receipt))
        .route("/receipts", get(get_receipts))
        .route("/receipts/{id}", get(get_receipt))
}
