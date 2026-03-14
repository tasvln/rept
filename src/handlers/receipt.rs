use axum::{
    Json,
    extract::{Extension, Multipart, State},
    http::StatusCode,
};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    models::receipt::Receipt, services::gemini_receipt::extract_receipt,
    states::server::ServerState, utils::jwt::Claims,
};

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn upload_receipt(
    State(state): State<ServerState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<Json<Receipt>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "Invalid user id in token".to_string(),
            }),
        )
    })?;

    let mut image_bytes = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        if name == "image" {
            image_bytes = field.bytes().await.unwrap().to_vec();
        }
    }

    if image_bytes.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: "No image provided".to_string(),
            }),
        ));
    }

    let receipt_data = extract_receipt(image_bytes, &state.gemini_key).await;

    let id = Uuid::new_v4();

    let purchase_date = receipt_data
        .purchase_date
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let return_by = receipt_data
        .return_by
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let warranty_until = receipt_data
        .warranty_until
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    sqlx::query(
        "INSERT INTO receipts (id, user_id, store_name, purchase_date, total, return_by, warranty_until)
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(id)
    .bind(user_id)
    .bind(&receipt_data.store_name)
    .bind(purchase_date)
    .bind(receipt_data.total)
    .bind(return_by)
    .bind(warranty_until)
    .execute(&state.db)
    .await
    .unwrap();

    let receipt = Receipt {
        id,
        user_id,
        store_name: receipt_data.store_name,
        purchase_date,
        total: receipt_data.total,
        return_by,
        warranty_until,
        image_url: None,
        raw_text: None,
    };

    Ok(Json(receipt))
}

pub async fn get_receipts(
    State(state): State<ServerState>,
    Extension(claims): Extension<Claims>,
) -> Json<Vec<Receipt>> {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    let receipts = sqlx::query_as::<_, Receipt>(
        "SELECT id, user_id, store_name, purchase_date, total, return_by, warranty_until, image_url, raw_text FROM receipts WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(receipts)
}

pub async fn get_receipt(
    State(state): State<ServerState>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Json<Receipt> {
    let user_id = Uuid::parse_str(&claims.sub).unwrap();

    let receipt = sqlx::query_as::<_, Receipt>(
        "SELECT id, user_id, store_name, purchase_date, total, return_by, warranty_until, image_url, raw_text FROM receipts WHERE id = $1 AND user_id = $2"
    )
    .bind(id)
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .unwrap();

    Json(receipt)
}
