//! Routing is declared here so it can be imported into the main and ran as a server, or be imported into the
//! lib so it can be compiled into another server.
//! 
//! # Notes
//! - All the logic is handled in the core, this is just wrapping the core logic around the functionality of
//!   the Actix web framework to serve HTTP requests to the core logic.
//! 
//! - it also must be noted that the "actix" feature is enabled for the crate "nanoservices-utils" meaning that
//!   all nanoservice errors are converted into HTTP responses for the actix framework meanging we can exploit
//!   the `?` operator. 
use nanoservices_utils::errors::NanoServiceError;
use actix_web::{
    HttpResponse, 
    web::{Json, ServiceConfig, post, scope, get}
};
use kernel::{CalcWork, Echo};
use core::{
    handle_contract_work,
    handle_contract_echo
};


/// HTTP wrapper for the Actix web framework to handle the contract work.
pub async fn contract_work(contract: Json<CalcWork>) -> Result<HttpResponse, NanoServiceError> {
    let contract = contract.into_inner();
    let result = handle_contract_work(contract).await?;
    Ok(HttpResponse::Ok().json(result))
}


/// HTTP wrapper for the Actix web framework to handle the contract echo.
pub async fn contract_echo(contract: Json<Echo>) -> Result<HttpResponse, NanoServiceError> {
    let contract = contract.into_inner();
    let result = handle_contract_echo(contract).await?;
    Ok(HttpResponse::Ok().json(result))
}


/// binds the endpoints to an Actix Server
pub fn endpoints_factory(app: &mut ServiceConfig) {
    app.service(
        scope("api/v1")
        .route("calc", get().to(contract_work))
        .route("calc", post().to(contract_work))
        .route("echo", get().to(contract_echo))
        .route("echo", post().to(contract_echo))
    );
}

