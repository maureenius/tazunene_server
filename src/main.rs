mod handlers;

use std::env;
use axum::{routing::{get, post}, Router};
use handlers::echo::{self, echo};

use crate::handlers::health_check;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = create_router();
    let listener = tokio::net::TcpListener::bind(
        &env::var("LISTENER_ADDR").expect("undefined [LISTENER_ADDR]")
    ).await.expect("failed to bind to address");

    axum::serve(listener, app).await.expect("failed to build server");
}

fn create_router() -> Router {
    let root = Router::new()
    .route("/", get(health_check::health_check));
    let messages = Router::new()
    .route("/echo", post(echo::echo));

    Router::new()
    .merge(root)
    .merge(messages)
}
