use axum::{
    routing::get,
    Router,
};

pub async fn start_rest_api_server() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    println!("Starting REST API server on port 2400");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2400").await.unwrap();
        axum::serve(listener, app).await.unwrap();
}
