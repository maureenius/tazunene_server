use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{domains::character, infrastructures::{open_ai_client::{ChatRequest, OpenAiClient}, repository::CharacterRepository}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSimpleRequest {
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSimpleResponse {
    message: String,
}

pub async fn chat_simple(
    client: State<Arc<OpenAiClient>>,
    Json(request): Json<ChatSimpleRequest>,
) -> anyhow::Result<Json<ChatSimpleResponse>, StatusCode> {
    match client.chat(&ChatRequest::new(request.message.clone().as_str()))
    .await {
        Ok(chat_response) => {
            let response = ChatSimpleResponse {
                message: chat_response.message,
            };
            Ok(Json(response))
        },
        Err(err) => {
            error!("Error processing request: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
