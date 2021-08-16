use actix_web::{get, App, HttpResponse, HttpServer, Responder};

pub mod models;
pub mod dtos;

pub mod controllers;
pub mod services;
pub mod kafka;
pub mod errors;

// APPLICATION CONFIG MODULES
pub mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:8002";
    let a = config::get_application_config();

    println!("===========================================");
    println!("{:?}", a);

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