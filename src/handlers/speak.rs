use std::{env, io::Write, sync::Arc};

use axum::{extract::State, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use vvcore::{AccelerationMode, VoicevoxCore};

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

fn create_vv(path: &str) -> VoicevoxCore {
    let dir = std::ffi::CString::new(path).unwrap();
    VoicevoxCore::new_from_options(AccelerationMode::Auto, 0, true, dir.as_c_str()).unwrap()
}
