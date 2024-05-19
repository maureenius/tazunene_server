mod handlers;
mod infrastructures;

use std::{env, sync::Arc};
use axum::{routing::{get, post}, Router};
use handlers::echo::{self};
use infrastructures::open_ai_client::{ApiKey, OpenAiClient};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::health_check;

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

    let root = Router::new()
    .route("/", get(health_check::health_check))
    .route("/echo", post(echo::echo));
    let messages = Router::new()
    .route("/chat_simple", post(handlers::chat_simple::chat_simple))
    .with_state(open_ai_client);

    Router::new()
    .merge(root)
    .merge(messages)
    .layer(CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
    )
}

fn create_open_ai_client(api_key: String) -> Arc<infrastructures::open_ai_client::OpenAiClient> {
    Arc::new(OpenAiClient::new(&ApiKey::new(api_key.as_str())))
}
