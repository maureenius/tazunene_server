use std::sync::Arc;

use axum::{body::Body, extract::State, response::{IntoResponse, Response}, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::infrastructures::voicevox_client::VoicevoxClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakRequest {
    message: String,
}

pub async fn speak(
    client: State<Arc<VoicevoxClient>>,
    Json(request): Json<SpeakRequest>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let sound = client.speak(request.message.as_str()).unwrap();

    let response = Response::builder()
        .header("Content-Type", "audio/wav")
        .body(Body::from(sound.as_slice().to_vec()))
        .expect("failed to build response");

    Ok(response)
}
