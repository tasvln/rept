use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Receipt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub store_name: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub total: Option<f64>,
    pub return_by: Option<NaiveDate>,
    pub warranty_until: Option<NaiveDate>,
    pub image_url: Option<String>,
    pub raw_text: Option<String>,
}
