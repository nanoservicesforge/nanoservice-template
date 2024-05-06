//! Routing is declared here so it can be imported into the main and ran as a server, or be imported into the
//! lib so it can be compiled into another server.
//! 
//! # Notes
//! - All the logic is handled in the core, this is just wrapping the core logic around the functionality of
//!   the Axum web framework to serve HTTP requests to the core logic.
//! 
//! - error handling for `NanoServiceError` is not implemented for Axum yet but will be in the future.
use axum::{
    routing::{get, post},
    extract::Json,
    Router,
    http::StatusCode
};
use kernel::{CalcWork, Echo};
use core::{
    handle_contract_work,
    handle_contract_echo
};


/// HTTP wrapper for the Axum web framework to handle the contract work.
async fn contract_work(Json(contract): Json<CalcWork>) -> (StatusCode, Json<CalcWork>) {
    let result = handle_contract_work(contract).await.unwrap();
    (StatusCode::OK, Json(result))
}


/// HTTP wrapper for the Axum web framework to handle the contract echo.
async fn contract_echo(Json(contract): Json<Echo>) -> (StatusCode, Json<Echo>) {
    let result = handle_contract_echo(contract).await.unwrap();
    (StatusCode::OK, Json(result))
}


/// binds the endpoints to an Axum Server
pub fn endpoints_factory() -> Router {
    Router::new()
        .nest("/api/v1", 
            axum::Router::new()
                .route("/calc", get(contract_work))
                .route("/calc", post(contract_work))
                .route("/echo", get(contract_echo))
                .route("/echo", post(contract_echo))
        )
}
