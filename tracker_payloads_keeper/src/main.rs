use std::sync::Arc;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:8003";

    // START HTTP SERVER WITH GLOBAL STATE
    HttpServer::new( move || {  
        App::new()
            .service(hi)
    })
    .bind(url)?
    .run()
    .await
}

/// Healthcheck endpoint
#[get("/info")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("
        TRACKER PAYLOADS KEEPER IS RUNNING
    ")
}