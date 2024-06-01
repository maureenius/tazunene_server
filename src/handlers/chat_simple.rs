use std::sync::Arc;

use axum::{Extension, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{domains::infra_trait::{CharacterRepository, TextGenerator}, usecases};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSimpleRequest {
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSimpleResponse {
    message: String,
}

pub async fn chat_simple<TG: TextGenerator, CR: CharacterRepository>(
    generator: Extension<Arc<TG>>,
    repository: Extension<Arc<CR>>,
    Json(request): Json<ChatSimpleRequest>,
) -> anyhow::Result<Json<ChatSimpleResponse>, StatusCode> {
    let chat_service = usecases::chat_service::ChatService::new(generator.0.clone(), repository.0.clone());

    match chat_service.generate_text(request.message)
    .await {
        Ok(chat_response) => {
            let response = ChatSimpleResponse {
                message: chat_response,
            };
            Ok(Json(response))
        },
        Err(err) => {
            error!("Error processing request: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
