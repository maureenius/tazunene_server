use std::sync::Arc;

use axum::{body::Body, extract::State, response::{IntoResponse, Response}, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::domains::infra_trait::VoiceSynthesizer;
use crate::usecases::speak_service::SpeakService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakRequest {
    message: String,
}

pub async fn speak<T: VoiceSynthesizer>(
    State(service): State<Arc<SpeakService<T>>>,
    Json(request): Json<SpeakRequest>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let sound = service.synthesize_speech(request.message.as_str()).unwrap();

    let response = Response::builder()
        .header("Content-Type", "audio/wav")
        .body(Body::from(sound.as_slice().to_vec()))
        .expect("failed to build response");

    Ok(response)
}
