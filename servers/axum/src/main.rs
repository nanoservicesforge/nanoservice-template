//! This is the entry point if we want to run our nanoservice as a standalone axum server.
mod routing;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    axum::serve(listener, routing::endpoints_factory()).await.unwrap();
}