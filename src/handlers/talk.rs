use std::sync::Arc;

use axum::{body::Body, extract::State, response::{IntoResponse, Response}, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{domains::infra_trait::{TextGenerator, VoiceSynthesizer}, usecases::speak_service::SpeakService};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalkRequest {
    message: String,
}

pub async fn talk<VS: VoiceSynthesizer, TG: TextGenerator>(
    State(speak_service): State<Arc<SpeakService<VS>>>,
    State(text_generator): State<Arc<TG>>,
    Json(request): Json<TalkRequest>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    Ok(
        Response::builder()
            .header("Content-Type", "audio/wav")
            .body(Body::from(
                speak_service.synthesize_speech(
                text_generator.generate(request.message).await
                .expect("failed to build talk text")
                .as_str()
            ).unwrap()))
            .expect("failed to build response")
    )
}
