use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EchoPayload {
    pub message: String,
}

pub async fn echo(Json(payload): Json<EchoPayload>) -> anyhow::Result<impl IntoResponse, StatusCode> {
    Ok(payload.message)
}
