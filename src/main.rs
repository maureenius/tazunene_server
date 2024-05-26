mod handlers;
mod infrastructures;
mod domains;
mod usecases;

use std::{env, sync::Arc};
use axum::{routing::{get, post}, Router};
use handlers::echo::{self};
use infrastructures::{open_ai_client::{ApiKey, OpenAiClient}, voicevox_client};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::health_check;
use crate::handlers::speak;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let app = create_router();
    let listener_addr = env::var("LISTENER_ADDR").expect("undefined [LISTENER_ADDR]");
    let listener = tokio::net::TcpListener::bind(&listener_addr).await.expect("failed to bind to address");

    tracing::info!("Starting server on {}", &listener_addr);
    axum::serve(listener, app).await.expect("failed to build server");
}

fn create_router() -> Router {
    let open_ai_client = create_open_ai_client(env::var("OPEN_AI_API_KEY").expect("undefined [OPEN_AI_API_KEY]"));
    let voicevox_client = voicevox_client::VoicevoxClient::new(env::var("OPEN_JTALK_PATH").expect("undefined [JTALK_PATH]").as_str());
    let speak_service = usecases::speak_service::SpeakService::new(voicevox_client);

    let root = Router::new()
    .route("/", get(health_check::health_check))
    .route("/echo", post(echo::echo));

    let speak = Router::new()
    .route("/speak", post(speak::speak))
    .with_state(Arc::new(speak_service));

    let messages = Router::new()
    .route("/chat_simple", post(handlers::chat_simple::chat_simple))
    .with_state(open_ai_client);

    Router::new()
    .merge(root)
    .merge(messages)
    .merge(speak)
    .layer(CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
    )
}

fn create_open_ai_client(api_key: String) -> Arc<infrastructures::open_ai_client::OpenAiClient> {
    Arc::new(OpenAiClient::new(&ApiKey::new(api_key.as_str())))
}
