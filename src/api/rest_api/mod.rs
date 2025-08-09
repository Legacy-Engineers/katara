pub mod users;

use axum::{Router, routing::get};
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub async fn start_rest_api_server(db: Surreal<Client>) {
    let db_instance = Arc::new(db);
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    println!("Starting REST API server on port 2400");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2400").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
