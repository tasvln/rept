use sqlx::PgPool;

#[derive(Clone)]
pub struct ServerState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub gemini_key: String,
}
