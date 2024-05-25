use std::{io::Write, sync::Arc};

use axum::{extract::State, Json};
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
) -> StatusCode {
    let sound = client.speak(request.message.as_str()).unwrap();

    let mut file = std::fs::File::create("audio.wav").unwrap();
    file.write_all(sound.as_slice()).unwrap();

    StatusCode::OK
}
