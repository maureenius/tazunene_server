mod handlers;
mod infrastructures;
mod domains;
mod usecases;

use std::{env, sync::Arc};
use axum::{routing::{get, post}, Extension, Router};
use handlers::echo::{self};
use infrastructures::{open_ai_client::{ApiKey, OpenAiClient}, repository::CharacterRepositoryPg, voicevox_client};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::health_check;
use crate::handlers::speak;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();
    let _db_pool = connect_db().await.expect("failed to connect to database");

    let app = create_router(_db_pool);
    let listener_addr = env::var("LISTENER_ADDR").expect("undefined [LISTENER_ADDR]");
    let listener = tokio::net::TcpListener::bind(&listener_addr).await.expect("failed to bind to address");

    tracing::info!("Starting server on {}", &listener_addr);
    axum::serve(listener, app).await.expect("failed to build server");
}

fn create_router(pool: PgPool) -> Router {
    let open_ai_client = create_open_ai_client(env::var("OPEN_AI_API_KEY").expect("undefined [OPEN_AI_API_KEY]"));
    let voicevox_client = voicevox_client::VoicevoxClient::new(env::var("OPEN_JTALK_PATH").expect("undefined [JTALK_PATH]").as_str());
    let character_repository = Arc::new(infrastructures::repository::CharacterRepositoryPg::new(pool));
    let speak_service = usecases::speak_service::SpeakService::new(voicevox_client);

    let root = Router::new()
    .route("/", get(health_check::health_check))
    .route("/echo", post(echo::echo));

    let speak = Router::new()
    .route("/speak", post(speak::speak))
    .with_state(Arc::new(speak_service));

    let messages = Router::new()
    .route("/chat", post(handlers::chat_simple::chat_simple::<OpenAiClient, CharacterRepositoryPg>))
    .layer(Extension(open_ai_client))
    .layer(Extension(character_repository));

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

async fn connect_db() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
    let db_url = env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
