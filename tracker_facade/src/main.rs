use std::sync::Arc;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

pub mod models;
pub mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:8002";

    // START HTTP SERVER WITH GLOBAL STATE
    HttpServer::new( move || {  
        App::new()
            .service(hi)
            .service(controllers::event_controller::post_event_async)
            .service(controllers::event_controller::post_events_async)
    })
    .bind(url)?
    .run()
    .await
}

/// Healthcheck endpoint
#[get("/info")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("
        TRACKER FACADE IS RUNNING
    ")
}