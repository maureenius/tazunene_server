use axum::{body::Body, http::{header, StatusCode}, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EchoPayload {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EchoResponse {
    pub message: String,
}

pub async fn echo(Json(payload): Json<EchoPayload>) -> anyhow::Result<Json<EchoResponse>, StatusCode> {
    info!("Received echo request with payload: {:?}", payload);

    let response = EchoResponse {
        message: payload.message.clone(),
    };

    Ok(Json(response))
}
