//! This is the entry point if we want to run our nanoservice as a standalone axum server.
use actix_web::{web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use actix_cors::Cors;

mod routing;


/// Basic web view in order to provide a health check which can
/// be accessed under the following endpoint with a GET method:
/// ```
/// /app/check
/// ``` 
async fn index() -> impl Responder {
    "Hello from Auth!"
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        App::new().service(

            web::scope("/app")
                .route("/check", web::get().to(index)),

        ).configure(routing::endpoints_factory).wrap(cors).wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
    })
        .bind("0.0.0.0:8001")?
        .run()
        .await
}