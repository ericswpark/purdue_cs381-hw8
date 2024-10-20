mod pages;
mod structs;

use crate::pages::*;

use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/1", post(question_one))
        .route("/1/test_cases", get(question_one_test_cases))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:10000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
